import type { PakWithLatestVersion } from "@paks/api";
import { TrendingUp, Download, ArrowRight, Package } from "lucide-react";

interface TrendingPaksProps {
  paks: PakWithLatestVersion[];
}

export function TrendingPaks({ paks }: TrendingPaksProps) {
  if (!paks || paks.length === 0) {
    return null;
  }


  return (
    <section className="py-16 px-4 sm:px-6 lg:px-8 relative overflow-hidden">
      {/* Background Accent - Orange Glow */}
      <div className="absolute inset-0 bg-gradient-to-b from-transparent via-orange-500/[0.03] to-transparent pointer-events-none" />
      
      <div className="max-w-6xl mx-auto relative">
        {/* Section Header */}
        <div className="flex items-center justify-between mb-8">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-orange-500/10 border border-orange-500/20" style={{ boxShadow: '0 0 15px oklch(0.7 0.15 50 / 20%)' }}>
              <TrendingUp className="w-5 h-5 text-orange-400" />
            </div>
            <div>
              <h2 className="text-2xl font-bold text-foreground">Trending Paks</h2>
              <p className="text-sm text-muted-foreground">Popular skills in the community</p>
            </div>
          </div>
          <a 
            href="/trending" 
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-orange-400 transition-colors group"
          >
            <span>View all</span>
            <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
          </a>
        </div>

        {/* Trending Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {paks.map((pak, index) => (
            <a
              key={pak.id}
              href={`/pak/${pak.owner_name}/${pak.name}`}
              className="group relative glass border border-border/30 hover:border-primary/40 p-5 transition-all duration-300 hover:-translate-y-1"
            >
              {/* Rank Badge - Orange */}
              <div className="absolute -top-2 -left-2 w-7 h-7 bg-orange-500/20 border border-orange-500/30 flex items-center justify-center" style={{ boxShadow: '0 0 10px oklch(0.7 0.15 50 / 25%)' }}>
                <span className="text-xs font-bold text-orange-400">#{index + 1}</span>
              </div>

              {/* Header */}
              <div className="flex items-start gap-3 mb-3">
                <div className="p-2 bg-muted/30 text-muted-foreground group-hover:text-primary group-hover:bg-primary/10 transition-colors">
                  <Package className="w-4 h-4" />
                </div>
                <div className="min-w-0 flex-1">
                  <div className="flex items-baseline gap-1">
                    <span className="text-xs text-muted-foreground">{pak.owner_name}/</span>
                  </div>
                  <h3 className="font-semibold text-foreground group-hover:text-primary transition-colors truncate">
                    {pak.name}
                  </h3>
                </div>
              </div>

              {/* Description */}
              {pak.description && (
                <p className="text-sm text-muted-foreground line-clamp-2 mb-4 leading-relaxed">
                  {pak.description}
                </p>
              )}

              {/* Footer Stats */}
              <div className="flex items-center gap-4 text-xs text-muted-foreground">
                <div className="flex items-center gap-1.5">
                  <Download className="w-3 h-3" />
                  <span>{formatNumber(pak.total_downloads)}</span>
                </div>
                {pak.latest_version && (
                  <span className="px-1.5 py-0.5 bg-primary/10 text-primary border border-primary/20 text-[10px]">
                    {pak.latest_version.version}
                  </span>
                )}
              </div>

              {/* Hover Accent - Orange */}
              <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-gradient-to-r from-transparent via-orange-500/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
            </a>
          ))}
        </div>
      </div>
    </section>
  );
}

function formatNumber(num: number): string {
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
  return String(num);
}
