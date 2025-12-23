import { 
  Download, 
  GitBranch, 
  Calendar, 
  Scale,
  ExternalLink,
  TrendingUp,
  User
} from "lucide-react";
import type { PakWithLatestVersion, Pak, PakVersion } from "@paks/api";

interface PakSidebarProps {
  pak: Pak | PakWithLatestVersion;
  latestVersion?: PakVersion | null;
}

function formatNumber(num: number): string {
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
  return String(num);
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function PakSidebar({ pak, latestVersion }: PakSidebarProps) {
  // Calculate download percentage for the bar (mock max for visual)
  const downloadPercent = Math.min(100, (pak.total_downloads / 10000) * 100);

  return (
    <div className="space-y-4">
      {/* Repository */}
      <SidebarCard title="Repository">
        <a
          href={pak.repository_url}
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-2 text-sm text-foreground hover:text-primary transition-colors group"
        >
          <GitBranch className="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
          <span className="truncate flex-1">
            {extractRepoName(pak.repository_url)}
          </span>
          <ExternalLink className="w-3 h-3 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity" />
        </a>
      </SidebarCard>

      {/* Downloads */}
      <SidebarCard title="Downloads">
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Download className="w-4 h-4 text-muted-foreground" />
              <span className="text-2xl font-bold text-foreground">
                {formatNumber(pak.total_downloads)}
              </span>
            </div>
            <span className="text-xs text-muted-foreground">all time</span>
          </div>
          
          {/* Download Bar - empty track is visible */}
          <div className="h-2 bg-muted/80 overflow-hidden rounded-sm">
            <div 
              className="h-full bg-gradient-to-r from-primary/60 via-primary to-primary/60 animate-gradient-shift"
              style={{ 
                width: `${downloadPercent}%`,
                backgroundSize: "200% 100%"
              }}
            />
          </div>

          {/* Weekly stats */}
          {"download_count" in pak && pak.download_count > 0 && (
            <div className="flex items-center gap-2 text-xs text-muted-foreground">
              <TrendingUp className="w-3 h-3 text-green-400" />
              <span>{formatNumber(pak.download_count)} this week</span>
            </div>
          )}
        </div>
      </SidebarCard>

      {/* Version Info */}
      {latestVersion && (
        <SidebarCard title="Version">
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-lg font-semibold text-foreground">
                {latestVersion.version}
              </span>
              <span className="px-1.5 py-0.5 text-[10px] bg-green-500/20 text-green-400 border border-green-500/30">
                latest
              </span>
            </div>
            {latestVersion.size_bytes && (
              <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <Scale className="w-3 h-3" />
                <span>{formatBytes(latestVersion.size_bytes)}</span>
              </div>
            )}
          </div>
        </SidebarCard>
      )}

      {/* Published Date */}
      {latestVersion && (
        <SidebarCard title="Last Published">
          <div className="flex items-center gap-2 text-sm text-foreground">
            <Calendar className="w-4 h-4 text-muted-foreground" />
            <span>{formatDate(latestVersion.published_at)}</span>
          </div>
        </SidebarCard>
      )}

      {/* License */}
      <SidebarCard title="License">
        <div className="flex items-center gap-2">
          <Scale className="w-4 h-4 text-muted-foreground" />
          <span className="text-sm text-foreground">MIT</span>
        </div>
      </SidebarCard>

      {/* Author */}
      <SidebarCard title="Author">
        <a
          href={`/search?query=${encodeURIComponent(pak.owner_name)}`}
          className="flex items-center gap-2 text-sm text-foreground hover:text-primary transition-colors group"
        >
          <User className="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
          <span>{pak.owner_name}</span>
        </a>
      </SidebarCard>
    </div>
  );
}

// Sidebar card wrapper
function SidebarCard({ 
  title, 
  children 
}: { 
  title: string; 
  children: React.ReactNode 
}) {
  return (
    <div className="glass border border-border/30 p-4">
      <h3 className="text-xs font-medium text-muted-foreground uppercase tracking-wider mb-3">
        {title}
      </h3>
      {children}
    </div>
  );
}

// Extract repo name from URL
function extractRepoName(url: string): string {
  try {
    const match = url.match(/github\.com\/([^/]+\/[^/]+)/);
    return match ? match[1] : url;
  } catch {
    return url;
  }
}

// Format bytes to human readable
function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
