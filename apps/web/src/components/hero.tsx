import { Package, Download, Upload, Zap, Code } from "lucide-react";
import { Button } from "@/components/ui/button";

export function Hero() {
  return (
    <section className="relative min-h-[calc(100vh-4rem)] flex items-center justify-center overflow-hidden">
      {/* Animated Background */}
      <div className="absolute inset-0 bg-grid animate-grid-pulse" />
      
      {/* Floating Orbs */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        {/* Large Cyan Orb - Top Right */}
        <div 
          className="absolute -top-32 -right-32 w-96 h-96 bg-primary/30 rounded-full animate-pulse-glow"
          style={{ animationDelay: '0s' }}
        />
        {/* Medium Orb - Bottom Left */}
        <div 
          className="absolute -bottom-24 -left-24 w-72 h-72 bg-cyan-500/20 rounded-full animate-pulse-glow"
          style={{ animationDelay: '2s' }}
        />
        {/* Small Accent Orb - Center Left */}
        <div 
          className="absolute top-1/3 left-1/4 w-32 h-32 bg-cyan-400/20 rounded-full animate-pulse-glow"
          style={{ animationDelay: '1s' }}
        />
      </div>

      {/* Floating Particles */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute top-1/4 left-1/3 w-2 h-2 bg-primary/60 animate-particle-float" style={{ animationDelay: '0s' }} />
        <div className="absolute top-1/2 right-1/4 w-1.5 h-1.5 bg-cyan-400/50 animate-particle-float" style={{ animationDelay: '1s' }} />
        <div className="absolute bottom-1/3 left-1/2 w-2.5 h-2.5 bg-cyan-300/40 animate-particle-float" style={{ animationDelay: '2s' }} />
        <div className="absolute top-2/3 right-1/3 w-1 h-1 bg-primary/70 animate-particle-float" style={{ animationDelay: '3s' }} />
        <div className="absolute top-1/5 right-1/2 w-2 h-2 bg-cyan-500/50 animate-particle-float" style={{ animationDelay: '4s' }} />
      </div>

      {/* Content */}
      <div className="relative z-10 mx-auto max-w-5xl px-4 sm:px-6 lg:px-8 text-center">
        {/* Badge */}
        <div className="inline-flex items-center gap-2 mb-8 px-4 py-2 glass border border-primary/30 animate-border-glow">
          <Package className="w-4 h-4 text-primary" />
          <span className="text-sm text-foreground/80">The Package Manager for AI Agent Skills</span>
        </div>

        {/* Main Headline */}
        <h1 className="text-5xl sm:text-6xl lg:text-7xl font-bold mb-6 leading-tight">
          <span className="block text-foreground">Discover</span>
          <span className="block gradient-text-animated">AI Agent Skills</span>
        </h1>

        {/* Subheadline */}
        <p className="text-lg sm:text-xl text-muted-foreground max-w-2xl mx-auto mb-10 leading-relaxed">
          Create, install, publish, and share reusable instruction sets that enhance 
          AI coding agents like Claude Code, Cursor, and Copilot.
        </p>

        {/* CTA Buttons */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16">
          <Button 
            size="lg" 
            className="h-12 px-8 text-sm bg-primary text-primary-foreground hover:bg-primary/90 glow-cyan transition-all hover:glow-cyan-lg group"
          >
            <Zap className="w-4 h-4 mr-2 transition-transform group-hover:scale-110" />
            Get Started
          </Button>
          <Button 
            variant="outline" 
            size="lg"
            className="h-12 px-8 text-sm border-border/50 hover:border-primary/50 hover:bg-accent/30 group"
          >
            <Code className="w-4 h-4 mr-2" />
            View on GitHub
            <span className="ml-2 text-muted-foreground group-hover:text-foreground transition-colors">→</span>
          </Button>
        </div>

        {/* Stats Row */}
        <div className="flex flex-wrap items-center justify-center gap-8 sm:gap-12 py-6 border-t border-border/30">
          <StatItem icon={<Package className="w-4 h-4" />} value="500+" label="Skills" />
          <StatItem icon={<Download className="w-4 h-4" />} value="10K+" label="Downloads" />
          <StatItem icon={<Upload className="w-4 h-4" />} value="100+" label="Publishers" />
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
      <div className="p-2 glass text-[oklch(0.75_0.16_55)] group-hover:text-[oklch(0.82_0.14_55)] transition-all" style={{ boxShadow: '0 0 10px oklch(0.75 0.16 55 / 12%)' }}>
        {icon}
      </div>
      <div className="text-left">
        <div className="text-xl font-bold text-foreground">{value}</div>
        <div className="text-xs text-muted-foreground">{label}</div>
      </div>
    </div>
  );
}
