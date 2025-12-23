import { Download, Calendar, Tag as TagIcon, GitBranch } from "lucide-react";
import type { Pak, PakWithLatestVersion, PakVersion } from "@paks/api";

interface VersionListProps {
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

function formatRelativeTime(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  
  if (diffDays === 0) return "today";
  if (diffDays === 1) return "yesterday";
  if (diffDays < 7) return `${diffDays} days ago`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
  if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`;
  return `${Math.floor(diffDays / 365)} years ago`;
}

export function VersionList({ pak, latestVersion }: VersionListProps) {
  // For now, we only have the latest version from the API
  // In a real implementation, we would fetch all versions
  const versions = latestVersion ? [latestVersion] : [];

  if (versions.length === 0) {
    return (
      <div className="glass border border-border/30 p-8">
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <TagIcon className="w-10 h-10 text-muted-foreground mb-4" />
          <h3 className="text-lg font-medium text-foreground mb-2">No Versions</h3>
          <p className="text-sm text-muted-foreground">
            No version history available.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="glass border border-border/30">
      {/* Header */}
      <div className="px-4 py-3 border-b border-border/30 bg-muted/20">
        <h3 className="text-sm font-medium text-foreground">Version History</h3>
        <p className="text-xs text-muted-foreground mt-0.5">
          {versions.length} version{versions.length !== 1 ? "s" : ""} published
        </p>
      </div>

      {/* Version List */}
      <div className="divide-y divide-border/20">
        {versions.map((version, index) => (
          <div 
            key={version.id} 
            className="px-4 py-4 hover:bg-muted/10 transition-colors"
          >
            <div className="flex items-start justify-between gap-4">
              {/* Version Info */}
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-2">
                  <span className="text-lg font-semibold text-foreground">
                    {version.version}
                  </span>
                  {index === 0 && (
                    <span className="px-1.5 py-0.5 text-[10px] bg-green-500/20 text-green-400 border border-green-500/30">
                      latest
                    </span>
                  )}
                </div>

                {/* Git Tag */}
                <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                  <GitBranch className="w-3.5 h-3.5" />
                  <code className="text-xs bg-muted/30 px-1.5 py-0.5">{version.git_tag}</code>
                </div>

                {/* Meta Row */}
                <div className="flex flex-wrap items-center gap-4 text-xs text-muted-foreground">
                  {/* Published Date */}
                  <div className="flex items-center gap-1.5">
                    <Calendar className="w-3 h-3" />
                    <span>{formatDate(version.published_at)}</span>
                    <span className="text-muted-foreground/50">
                      ({formatRelativeTime(version.published_at)})
                    </span>
                  </div>

                  {/* Downloads */}
                  <div className="flex items-center gap-1.5">
                    <Download className="w-3 h-3" />
                    <span>{formatNumber(version.downloads)} downloads</span>
                  </div>
                </div>
              </div>

              {/* Install Command */}
              <div className="flex-shrink-0">
                <code className="text-xs bg-muted/30 text-muted-foreground px-2 py-1 hidden sm:block">
                  paks install {pak.owner_name}/{pak.name}@{version.version}
                </code>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Note about more versions */}
      {versions.length === 1 && (
        <div className="px-4 py-3 border-t border-border/30 bg-muted/10">
          <p className="text-xs text-muted-foreground text-center">
            Full version history will be available in a future update.
          </p>
        </div>
      )}
    </div>
  );
}
