import { useState } from "react";
import { Copy, Check, Terminal } from "lucide-react";

interface InstallCommandProps {
  uri: string;
}

export function InstallCommand({ uri }: InstallCommandProps) {
  const [copied, setCopied] = useState(false);
  const command = `paks install ${uri}`;

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
    <div className="glass border border-border/30 hover:border-primary/30 transition-colors group animate-border-glow">
      <div className="flex items-center justify-between px-4 py-3 pb-0 border-b border-border/20">
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <Terminal className="w-3.5 h-3.5" />
          <span>Install</span>
        </div>
      </div>
      
      <div className="flex items-center gap-3 px-4 py-4 pt-2">
        {/* Command Display */}
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
