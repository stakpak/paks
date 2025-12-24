import { useState } from "react";
import { Copy, Check, Terminal, ChevronDown } from "lucide-react";

interface InstallCommandProps {
  uri: string;
}

const agents = [
  { id: "stakpak", name: "Stakpak", flag: null },
  { id: "claude-code", name: "Claude Code", flag: "claude-code" },
] as const;

type AgentId = typeof agents[number]["id"];

export function InstallCommand({ uri }: InstallCommandProps) {
  const [copied, setCopied] = useState(false);
  const [selectedAgent, setSelectedAgent] = useState<AgentId>("stakpak");
  const [dropdownOpen, setDropdownOpen] = useState(false);

  const agent = agents.find((a) => a.id === selectedAgent) ?? agents[0];
  const command = agent.flag 
    ? `paks install ${uri} --agent ${agent.flag}`
    : `paks install ${uri}`;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(command);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  return (
    <div className="glass border border-border/30 hover:border-primary/30 transition-colors group relative z-20 overflow-visible">
      {/* Header with Agent Selector */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-border/20">
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <Terminal className="w-3.5 h-3.5" />
          <span>Install</span>
        </div>
        
        {/* Agent Dropdown */}
        <div className="relative">
          <button
            onClick={() => setDropdownOpen(!dropdownOpen)}
            onBlur={() => setTimeout(() => setDropdownOpen(false), 150)}
            className="flex items-center gap-1.5 px-2 py-1 text-xs text-muted-foreground hover:text-foreground bg-muted/30 hover:bg-muted/50 border border-border/30 transition-colors"
          >
            <span>{agent.name}</span>
            <ChevronDown className={`w-3 h-3 transition-transform ${dropdownOpen ? "rotate-180" : ""}`} />
          </button>
          
          {dropdownOpen && (
            <div className="absolute right-0 top-full mt-1 z-50 min-w-[140px] py-1 bg-background border border-border/50 shadow-lg">
              {agents.map((a) => (
                <button
                  key={a.id}
                  onClick={() => {
                    setSelectedAgent(a.id);
                    setDropdownOpen(false);
                  }}
                  className={`w-full text-left px-3 py-1.5 text-xs transition-colors ${
                    a.id === selectedAgent
                      ? "bg-primary/20 text-primary"
                      : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
                  }`}
                >
                  {a.name}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
      
      {/* Command Display */}
      <div className="flex items-center gap-3 px-4 py-3">
        <div className="flex-1 flex items-center gap-3 overflow-x-auto scrollbar-thin">
          <span className="text-primary/60 select-none">$</span>
          <code className="font-mono text-sm text-foreground whitespace-nowrap">
            {command}
          </code>
        </div>

        {/* Copy Button */}
        <button
          onClick={handleCopy}
          className={`
            flex-shrink-0 relative p-2 transition-all duration-300
            ${copied 
              ? "bg-green-500/20 text-green-400 glow-cyan-sm" 
              : "bg-primary/10 text-muted-foreground hover:text-primary hover:bg-primary/20"}
          `}
          title={copied ? "Copied!" : "Copy to clipboard"}
        >
          <div className={`transition-all duration-300 ${copied ? "scale-110" : "scale-100"}`}>
            {copied ? (
              <Check className="w-4 h-4" />
            ) : (
              <Copy className="w-4 h-4" />
            )}
          </div>
        </button>
      </div>
    </div>
  );
}
