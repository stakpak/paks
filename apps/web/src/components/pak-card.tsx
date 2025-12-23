import { Download, ExternalLink, GitBranch, Tag } from "lucide-react";
import type { Pak, PakWithLatestVersion } from "@paks/api";

function formatNumber(num: number): string {
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
  return String(num);
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

interface PakCardProps {
  pak: Pak | PakWithLatestVersion;
}

export function PakCard({ pak }: PakCardProps) {
  const latestVersion = "latest_version" in pak ? pak.latest_version : null;
  
  return (
    <article className="group relative p-5 glass hover-lift transition-all duration-300 border border-border/30 hover:border-primary/40">
      {/* Header: Name and Owner */}
      <div className="flex items-start justify-between gap-4 mb-3">
        <div className="min-w-0 flex-1">
          <a 
            href={`/pak/${pak.owner_name}/${pak.name}`}
            className="inline-flex items-baseline gap-1.5 group/link"
          >
            <span className="text-muted-foreground text-sm">{pak.owner_name}/</span>
            <h3 className="text-base font-semibold text-foreground group-hover/link:text-primary transition-colors truncate">
              {pak.name}
            </h3>
          </a>
          
          {/* Version Badge */}
          {latestVersion && (
            <span className="ml-2 inline-flex items-center gap-1 px-1.5 py-0.5 text-[10px] bg-primary/10 text-primary border border-primary/20">
              <Tag className="w-2.5 h-2.5" />
              v{latestVersion.version}
            </span>
          )}
        </div>

        {/* Repository Link */}
        <a
          href={pak.repository_url}
          target="_blank"
          rel="noopener noreferrer"
          className="flex-shrink-0 p-1.5 text-muted-foreground hover:text-primary transition-colors"
          title="View repository"
        >
          <GitBranch className="w-4 h-4" />
        </a>
      </div>

      {/* Description */}
      {pak.description && (
        <p className="text-sm text-muted-foreground mb-4 line-clamp-2 leading-relaxed">
          {pak.description}
        </p>
      )}

      {/* Tags */}
      {pak.tags && pak.tags.length > 0 && (
        <div className="flex flex-wrap gap-1.5 mb-4">
          {pak.tags.slice(0, 5).map((tag: string) => (
            <span
              key={tag}
              className="px-2 py-0.5 text-[10px] bg-accent/50 text-accent-foreground border border-border/30 hover:border-primary/30 hover:bg-primary/10 transition-colors cursor-pointer"
            >
              {tag}
            </span>
          ))}
          {pak.tags.length > 5 && (
            <span className="px-2 py-0.5 text-[10px] text-muted-foreground">
              +{pak.tags.length - 5} more
            </span>
          )}
        </div>
      )}

      {/* Footer: Stats */}
      <div className="flex items-center gap-4 text-xs text-muted-foreground pt-3 border-t border-border/20">
        {/* Downloads */}
        <div className="flex items-center gap-1.5" title="Downloads">
          <Download className="w-3.5 h-3.5" />
          <span>{formatNumber(pak.total_downloads)}</span>
        </div>

        {/* Published Date */}
        {latestVersion && (
          <div className="flex items-center gap-1.5">
            <span>published {formatRelativeTime(latestVersion.published_at)}</span>
          </div>
        )}

        {/* Spacer */}
        <div className="flex-1" />

        {/* View Link */}
        <a
          href={`/pak/${pak.owner_name}/${pak.name}`}
          className="inline-flex items-center gap-1 text-primary/80 hover:text-primary transition-colors opacity-0 group-hover:opacity-100"
        >
          View
          <ExternalLink className="w-3 h-3" />
        </a>
      </div>

      {/* Hover Accent Line */}
      <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-gradient-to-r from-transparent via-primary/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
    </article>
  );
}
