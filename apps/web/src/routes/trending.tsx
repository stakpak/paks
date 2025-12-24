import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { PaksClient } from "@paks/api";
import { PakCard } from "@/components/pak-card";
import { Pagination } from "@/components/pagination";
import { Footer } from "@/components/footer";
import { TrendingUp, Loader2, AlertCircle, Package } from "lucide-react";

const RESULTS_PER_PAGE = 20;

interface TrendingParams {
  page?: number;
}

export const Route = createFileRoute("/trending")({
  validateSearch: (search: Record<string, unknown>): TrendingParams => {
    return {
      page: typeof search.page === "number" ? search.page : 1,
    };
  },
  component: TrendingPage,
});

function TrendingPage() {
  const navigate = useNavigate();
  const { page = 1 } = Route.useSearch();

  const client = new PaksClient();

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["trendingPaks", page],
    queryFn: () =>
      client.listPaks({
        sort_by: "TRENDING",
        limit: RESULTS_PER_PAGE,
        offset: (page - 1) * RESULTS_PER_PAGE,
      }),
    staleTime: 30000,
  });

  const handlePageChange = (newPage: number) => {
    navigate({
      to: "/trending",
      search: { page: newPage },
    });
  };

  // Estimate total pages (API doesn't return total count, so we estimate)
  const hasMore = data && data.length === RESULTS_PER_PAGE;
  const totalPages = hasMore ? page + 1 : page;

  return (
    <div className="min-h-screen flex flex-col">
      <main className="flex-1 py-8 px-4 sm:px-6 lg:px-8">
        <div className="max-w-5xl mx-auto">
          {/* Header */}
          <div className="mb-8">
            <div className="flex items-center gap-3 mb-2">
              <div className="p-2 bg-primary/10 text-primary border border-primary/20">
                <TrendingUp className="w-5 h-5" />
              </div>
              <h1 className="text-2xl font-bold text-foreground">
                Trending Paks
              </h1>
            </div>
            <p className="text-sm text-muted-foreground ml-12">
              The most popular AI agent skills in the community
            </p>
            {data && (
              <p className="text-sm text-muted-foreground ml-12 mt-1">
                {data.length === 0
                  ? "No paks found"
                  : `Showing ${data.length} pak${data.length !== 1 ? "s" : ""}`}
              </p>
            )}
          </div>

          {/* Loading State */}
          {isLoading && (
            <div className="flex flex-col items-center justify-center py-20">
              <Loader2 className="w-8 h-8 text-primary animate-spin mb-4" />
              <p className="text-muted-foreground">Loading trending paks...</p>
            </div>
          )}

          {/* Error State */}
          {isError && (
            <div className="flex flex-col items-center justify-center py-20 glass p-8">
              <AlertCircle className="w-10 h-10 text-destructive mb-4" />
              <h2 className="text-lg font-semibold text-foreground mb-2">Something went wrong</h2>
              <p className="text-sm text-muted-foreground text-center max-w-md">
                {error instanceof Error ? error.message : "Failed to load paks. Please try again."}
              </p>
            </div>
          )}

          {/* Empty State */}
          {!isLoading && !isError && data && data.length === 0 && (
            <div className="flex flex-col items-center justify-center py-20 glass p-8">
              <Package className="w-12 h-12 text-muted-foreground mb-4" />
              <h2 className="text-lg font-semibold text-foreground mb-2">No paks yet</h2>
              <p className="text-sm text-muted-foreground text-center max-w-md">
                No trending paks available yet. Be the first to publish!
              </p>
            </div>
          )}

          {/* Results Grid */}
          {!isLoading && !isError && data && data.length > 0 && (
            <>
              <div className="grid gap-4">
                {data.map((pak) => (
                  <PakCard key={pak.id} pak={pak} />
                ))}
              </div>

              {/* Pagination */}
              <Pagination
                currentPage={page}
                totalPages={totalPages}
                onPageChange={handlePageChange}
              />
            </>
          )}
        </div>
      </main>

      <Footer />
    </div>
  );
}
