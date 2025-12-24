import { useState } from "react";
import { Copy, Check } from "lucide-react";

interface InstallCommandProps {
  uri: string;
}

const agents = [
  // { id: "stakpak", name: "Stakpak", flag: "stakpak" },
  { id: "claude-code", name: "Claude Code", flag: "claude-code" },
  { id: "codex", name: "Codex", flag: "codex" },
  { id: "cursor", name: "Cursor", flag: "cursor" },
  { id: "copilot", name: "Copilot", flag: "copilot" },
  { id: "goose", name: "Goose", flag: "goose" },
  { id: "opencode", name: "OpenCode", flag: "opencode" },
  { id: "amp", name: "AMP", flag: "amp" },
] as const;

type AgentId = typeof agents[number]["id"];

export function InstallCommand({ uri }: InstallCommandProps) {
  const [copied, setCopied] = useState(false);
  const [selectedAgent, setSelectedAgent] = useState<AgentId>("claude-code");

  const agent = agents.find((a) => a.id === selectedAgent) ?? agents[0];
  const command = `paks install ${uri} --agent ${agent.flag}`;

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
    <div className="glass border border-border/30 hover:border-primary/30 transition-colors group">
      {/* Agent Tabs Header */}
      <div className="flex items-start justify-between px-3 py-2 border-b border-border/20 gap-2">
        <div className="flex flex-wrap gap-1 flex-1">
          {agents.map((a) => (
            <button
              key={a.id}
              onClick={() => setSelectedAgent(a.id)}
              className={`px-2.5 py-1 text-[11px] whitespace-nowrap transition-colors flex-shrink-0 ${
                a.id === selectedAgent
                  ? "bg-[oklch(0.75_0.16_55/15%)] text-[oklch(0.75_0.16_55)] font-medium"
                  : "text-muted-foreground hover:text-foreground"
              }`}
            >
              {a.name}
            </button>
          ))}
        </div>
        
        {/* Copy Button */}
        <button
          onClick={handleCopy}
          className={`flex-shrink-0 ml-2 p-1.5 transition-all duration-300 ${
            copied 
              ? "bg-green-500/20 text-green-400" 
              : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
          }`}
          title={copied ? "Copied!" : "Copy to clipboard"}
        >
          {copied ? (
            <Check className="w-3.5 h-3.5" />
          ) : (
            <Copy className="w-3.5 h-3.5" />
          )}
        </button>
      </div>
      
      {/* Command Display */}
      <div className="flex items-center gap-3 px-4 py-3 min-w-0">
        <div className="flex items-center gap-3 overflow-x-auto scrollbar-thin min-w-0">
          <span className="text-primary/60 select-none flex-shrink-0">$</span>
          <code className="font-mono text-sm text-foreground whitespace-nowrap">
            {command}
          </code>
        </div>
      </div>
    </div>
  );
}

