import { useState, useEffect, useRef, useCallback, useMemo, useLayoutEffect } from "react";
import { Package, Download, Upload, Zap, Rocket, Github } from "lucide-react";

// ASCII characters to use (package/code themed)
const ASCII_CHARS = ['#', '@', '*', '+', '=', '-', '.', ':', ';', '/', '\\', '|', '{', '}', '[', ']', '<', '>', '0', '1'];

interface AsciiCell {
  char: string;
  x: number;
  y: number;
}

interface TrailPoint {
  x: number;
  y: number;
  timestamp: number;
}

export function Hero() {
  const containerRef = useRef<HTMLElement>(null);
  const [mousePos, setMousePos] = useState({ x: -1000, y: -1000 });
  const [trail, setTrail] = useState<TrailPoint[]>([]);
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });
  const [isHovering, setIsHovering] = useState(false);

  // Grid settings
  const cellSize = 24;
  const colorRadius = 120; // Comet head radius

  // Generate ASCII grid (no displacement, just static positions)
  const asciiGrid = useMemo(() => {
    if (dimensions.width === 0 || dimensions.height === 0) return [];
    
    const cols = Math.ceil(dimensions.width / cellSize) + 2;
    const rows = Math.ceil(dimensions.height / cellSize) + 2;
    const grid: AsciiCell[] = [];
    
    for (let row = 0; row < rows; row++) {
      for (let col = 0; col < cols; col++) {
        grid.push({
          char: ASCII_CHARS[Math.floor(Math.random() * ASCII_CHARS.length)],
          x: col * cellSize,
          y: row * cellSize,
        });
      }
    }
    
    return grid;
  }, [dimensions.width, dimensions.height, cellSize]);

  // Calculate if a cell is inside the comet shape (only when moving - trail exists)
  // Returns distance and fade opacity based on trail age
  const getCellCometInfo = (cellX: number, cellY: number): { distance: number; inTrail: boolean; fade: number } => {
    const trailLifetime = 800; // Must match trail cleanup timeout
    const now = Date.now();
    
    // No effect if trail is empty (mouse is stationary)
    if (trail.length === 0) {
      return { distance: Infinity, inTrail: false, fade: 0 };
    }
    
    // Calculate head fade based on newest trail point age
    const newestPoint = trail[trail.length - 1];
    const headAge = now - newestPoint.timestamp;
    const headFade = Math.max(0, 1 - headAge / trailLifetime);
    
    // Check distance from cursor (comet head) - only if trail exists
    const headDx = cellX - mousePos.x;
    const headDy = cellY - mousePos.y;
    const headDistance = Math.sqrt(headDx * headDx + headDy * headDy);
    
    if (headDistance < colorRadius) {
      return { distance: headDistance, inTrail: false, fade: headFade };
    }
    
    // Check if in trail - with age-based fade
    for (let i = 0; i < trail.length - 1; i++) {
      const t = i / Math.max(1, trail.length - 1);
      const trailRadius = colorRadius * t; // Trail tapers from 0 to colorRadius
      
      const trailDx = cellX - trail[i].x;
      const trailDy = cellY - trail[i].y;
      const trailDist = Math.sqrt(trailDx * trailDx + trailDy * trailDy);
      
      if (trailDist < trailRadius) {
        // Fade based on this trail point's age
        const pointAge = now - trail[i].timestamp;
        const pointFade = Math.max(0, 1 - pointAge / trailLifetime);
        
        return { 
          distance: (trailDist / trailRadius) * colorRadius, 
          inTrail: true, 
          fade: pointFade 
        };
      }
    }
    
    return { distance: Infinity, inTrail: false, fade: 0 };
  };

  // Update dimensions on mount and resize - use layoutEffect for immediate measurement
  useLayoutEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect();
        setDimensions({ width: rect.width, height: rect.height });
      }
    };
    
    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  // Handle mouse move - update position and trail
  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLElement>) => {
    if (!containerRef.current) return;
    const rect = containerRef.current.getBoundingClientRect();
    const newPos = {
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    };
    setMousePos(newPos);
    
    // Add to trail - longer trail with slower fade
    setTrail(prev => {
      const now = Date.now();
      // Keep last 30 points and remove old ones (> 800ms for slower fade)
      const filtered = prev.filter(p => now - p.timestamp < 800).slice(-30);
      return [...filtered, { ...newPos, timestamp: now }];
    });
  }, []);

  // Clean up old trail points
  useEffect(() => {
    if (!isHovering) {
      setTrail([]);
      return;
    }
    
    const interval = setInterval(() => {
      const now = Date.now();
      setTrail(prev => prev.filter(p => now - p.timestamp < 800)); // Slower fade
    }, 100); // Less frequent cleanup
    
    return () => clearInterval(interval);
  }, [isHovering]);

  return (
    <section 
      ref={containerRef}
      className="relative min-h-[calc(100vh-4rem)] flex items-center justify-center overflow-hidden"
      onMouseMove={handleMouseMove}
      onMouseEnter={() => setIsHovering(true)}
      onMouseLeave={() => {
        setIsHovering(false);
        setMousePos({ x: -1000, y: -1000 });
        setTrail([]);
      }}
    >
      {/* ASCII Background - Colorized by comet (head + trail) */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none select-none">
        {asciiGrid.map((cell, index) => {
          const { distance, inTrail, fade } = getCellCometInfo(cell.x, cell.y);
          const isInComet = distance < colorRadius && isHovering && fade > 0;
          
          // Apply fade to brightness for smooth disappear effect
          const baseBrightness = isInComet
            ? 0.5 + (1 - distance / colorRadius) * 0.5
            : 0.12;
          const brightness = isInComet ? baseBrightness * fade : 0.12;
          
          // Gradient effect: orange at center, cyan at edges
          const normalizedDist = isInComet ? distance / colorRadius : 1;
          const orangeThreshold = 0.7;
          let hue: number;
          if (isInComet) {
            if (normalizedDist < orangeThreshold) {
              hue = 55; // Our orange
            } else {
              hue = 55 + ((normalizedDist - orangeThreshold) / (1 - orangeThreshold)) * 140;
            }
            // Trail has slightly more cyan
            if (inTrail) hue += 20;
          } else {
            hue = 195; // Default cyan
          }
          const chroma = isInComet ? 0.18 * fade : 0.1;
          
          return (
            <span
              key={index}
              className="absolute font-mono text-xs"
              style={{
                left: cell.x,
                top: cell.y,
                color: `oklch(${0.5 + brightness * 0.4} ${chroma} ${hue} / ${brightness})`,
                textShadow: isInComet 
                  ? `0 0 10px oklch(0.75 0.2 ${hue} / ${brightness * 0.6 * fade})`
                  : 'none',
              }}
            >
              {cell.char}
            </span>
          );
        })}
      </div>

      {/* Scanlines overlay */}
      <div 
        className="absolute inset-0 pointer-events-none opacity-[0.03]"
        style={{
          background: 'repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(0,212,255,0.5) 2px, rgba(0,212,255,0.5) 4px)',
        }}
      />

      {/* Floating Orbs/Circles - with staggered but faster animation start */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div 
          className="absolute -top-48 -right-48 w-[600px] h-[600px] rounded-full animate-pulse-glow"
          style={{ background: 'radial-gradient(circle, oklch(0.6 0.18 195 / 35%) 0%, transparent 70%)' }}
        />
        <div 
          className="absolute -bottom-32 -left-32 w-[500px] h-[500px] rounded-full animate-pulse-glow"
          style={{ background: 'radial-gradient(circle, oklch(0.55 0.15 210 / 28%) 0%, transparent 70%)', animationDelay: '0.5s' }}
        />
        <div 
          className="absolute top-1/4 left-[10%] w-64 h-64 rounded-full animate-pulse-glow"
          style={{ background: 'radial-gradient(circle, oklch(0.65 0.12 180 / 22%) 0%, transparent 70%)', animationDelay: '0.2s' }}
        />
        <div 
          className="absolute -top-24 left-1/4 w-72 h-72 rounded-full animate-pulse-glow"
          style={{ background: 'radial-gradient(circle, oklch(0.7 0.18 50 / 20%) 0%, transparent 70%)', animationDelay: '0.8s' }}
        />
        <div 
          className="absolute bottom-1/4 -right-16 w-52 h-52 rounded-full animate-pulse-glow"
          style={{ background: 'radial-gradient(circle, oklch(0.6 0.15 195 / 22%) 0%, transparent 70%)', animationDelay: '0.3s' }}
        />
        <div 
          className="absolute top-1/2 right-[15%] w-40 h-40 rounded-full animate-pulse-glow"
          style={{ background: 'radial-gradient(circle, oklch(0.65 0.15 45 / 18%) 0%, transparent 70%)', animationDelay: '0.6s' }}
        />
      </div>


      {/* Content */}
      <div className="relative z-10 mx-auto max-w-5xl px-4 sm:px-6 lg:px-8 text-center">
        {/* Badge */}
        {/* <div 
          className="inline-flex items-center gap-2 mb-8 px-4 py-2 border-2 border-primary/30 bg-background/80 backdrop-blur-sm"
          style={{ boxShadow: '4px 4px 0 rgba(0,212,255,0.2)' }}
        >
          <Package className="w-4 h-4 text-primary" />
          <span className="text-sm text-foreground/80 font-mono">The Package Manager for AI Agent Skills</span>
        </div> */}

        {/* Main Headline */}
        <h1 className="text-5xl sm:text-6xl lg:text-7xl font-bold mb-6 leading-tight">
          <span className="block text-foreground">Discover</span>
          <span className="block gradient-text-animated">New Agent Skills</span>
        </h1>

        {/* Subheadline */}
        <p className="text-lg sm:text-xl text-muted-foreground max-w-2xl mx-auto mb-10 leading-relaxed">
          Publish, Find, and Install{" "}
          <a 
            href="https://agentskills.io/home" 
            target="_blank" 
            rel="noopener noreferrer"
            className="text-primary hover:text-primary/80 underline underline-offset-2 decoration-primary/50 hover:decoration-primary transition-colors"
          >
            Agent Skills
          </a>{" "}
         to enhance agents like Claude Code, Cursor, Copilot, and more...
        </p>

        {/* CTA Buttons */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16">
          <a 
            href="https://github.com/stakpak/paks?tab=readme-ov-file#installation"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center h-12 px-8 text-sm font-medium font-mono bg-primary text-primary-foreground hover:bg-primary/90 transition-all group"
            style={{ boxShadow: '4px 4px 0 rgba(0,100,120,0.5)' }}
          >
            <Rocket className="w-4 h-4 mr-2 transition-transform group-hover:scale-110" />
            Get Started
          </a>
          <a 
            href="https://github.com/stakpak/paks"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center h-12 px-8 text-sm font-medium font-mono border-2 border-border/50 bg-background/50 hover:border-primary/50 hover:bg-accent/30 transition-colors group"
            style={{ boxShadow: '4px 4px 0 rgba(255,255,255,0.1)' }}
          >
            <Github className="w-4 h-4 mr-2" />
            View on GitHub
            <span className="ml-2 text-muted-foreground group-hover:text-foreground transition-colors">→</span>
          </a>
        </div>

        {/* Quick Commands Box */}
        <QuickCommands />

        {/* Stats Row */}
        <div className="flex flex-wrap items-center justify-center gap-8 sm:gap-12 py-6 border-t-2 border-dashed border-border/30">
          <StatItem icon={<Package className="w-4 h-4" />} value="50+" label="Skills" />
          <StatItem icon={<Download className="w-4 h-4" />} value="100+" label="Downloads" />
          <StatItem icon={<Upload className="w-4 h-4" />} value="10+" label="Publishers" />
          <StatItem icon={<Zap className="w-4 h-4" />} value="8" label="AGENTS" />
        </div>
      </div>

      {/* Bottom Gradient Fade */}
      <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-background to-transparent pointer-events-none" />
    </section>
  );
}

function StatItem({ icon, value, label }: { icon: React.ReactNode; value: string; label: string }) {
  return (
    <div className="flex items-center gap-3 group">
      <div 
        className="p-2 border-2 border-orange-500/30 bg-background/50 text-orange-400 group-hover:border-orange-500/50 transition-all font-mono"
        style={{ boxShadow: '2px 2px 0 rgba(249,115,22,0.15)' }}
      >
        {icon}
      </div>
      <div className="text-left">
        <div className="text-xl font-bold text-foreground font-mono">{value}</div>
        <div className="text-xs text-muted-foreground font-mono uppercase tracking-wider">{label}</div>
      </div>
    </div>
  );
}

import { useState as useStateLocal } from "react";
import { Copy, Check, Search, Terminal, Upload as UploadIcon } from "lucide-react";

const commands = [
  { 
    id: "search", 
    name: "Search", 
    icon: Search,
    command: "paks search \"aws\"",
    description: "Find skills for your agent"
  },
  { 
    id: "install", 
    name: "Install", 
    icon: Terminal,
    command: "paks install stakpak/terraform --agent claude-code",
    description: "Add skills to your project"
  },
  { 
    id: "publish", 
    name: "Publish", 
    icon: UploadIcon,
    command: "paks publish ./my-skill",
    description: "Share your skills with the world"
  },
] as const;

type CommandId = typeof commands[number]["id"];

function QuickCommands() {
  const [selectedTab, setSelectedTab] = useStateLocal<CommandId>("search");
  const [copied, setCopied] = useStateLocal(false);

  const currentCommand = commands.find((c) => c.id === selectedTab) ?? commands[0];

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(currentCommand.command);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  return (
    <div 
      className="max-w-2xl mx-auto mb-12 bg-black/40 border-2 border-border/40 backdrop-blur-sm"
      style={{ boxShadow: '6px 6px 0 rgba(0,212,255,0.15)' }}
    >
      {/* Tabs Header */}
      <div className="flex items-center justify-between border-b border-border/30">
        <div className="flex">
          {commands.map((cmd) => {
            const Icon = cmd.icon;
            const isActive = cmd.id === selectedTab;
            return (
              <button
                key={cmd.id}
                onClick={() => setSelectedTab(cmd.id)}
                className={`flex items-center gap-2 px-4 py-3 text-sm font-mono transition-all border-b-2 -mb-[2px] ${
                  isActive
                    ? "border-primary text-primary bg-primary/10"
                    : "border-transparent text-muted-foreground hover:text-foreground hover:bg-white/5"
                }`}
              >
                <Icon className="w-4 h-4" />
                {cmd.name}
              </button>
            );
          })}
        </div>
        
        {/* Copy Button */}
        <button
          onClick={handleCopy}
          className={`mr-3 p-2 transition-all duration-300 ${
            copied 
              ? "bg-green-500/20 text-green-400" 
              : "text-muted-foreground hover:text-foreground hover:bg-white/10"
          }`}
          title={copied ? "Copied!" : "Copy to clipboard"}
        >
          {copied ? (
            <Check className="w-4 h-4" />
          ) : (
            <Copy className="w-4 h-4" />
          )}
        </button>
      </div>
      
      {/* Command Display */}
      <div className="px-5 py-4">
        <div className="flex items-center gap-3">
          <span className="text-primary/60 select-none font-mono">$</span>
          <code className="font-mono text-sm text-foreground">
            {currentCommand.command}
          </code>
        </div>
        {/* <p className="mt-2 text-xs text-muted-foreground font-mono pl-6">
          {currentCommand.description}
        </p> */}
      </div>
    </div>
  );
}
