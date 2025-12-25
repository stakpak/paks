import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { PaksClient } from "@paks/api";
import { PakCard } from "@/components/pak-card";
import { Pagination } from "@/components/pagination";
import { Footer } from "@/components/footer";
import { Search, Package, Loader2, AlertCircle } from "lucide-react";

const RESULTS_PER_PAGE = 20;

interface SearchParams {
  query?: string;
  page?: number;
}

export const Route = createFileRoute("/search")({
  validateSearch: (search: Record<string, unknown>): SearchParams => {
    return {
      query: typeof search.query === "string" ? search.query : undefined,
      page: typeof search.page === "number" ? search.page : 1,
    };
  },
  head: () => {
    // Static SEO for search page - dynamic content handled client-side
    const title = "Search Packages - Paks";
    const description = "Search and discover AI agent skills and packages. Find the perfect tools for Claude Code, Cursor, GitHub Copilot and other coding agents.";
    
    return {
      meta: [
        { title },
        { name: "description", content: description },
        { name: "robots", content: "noindex, follow" }, // Search pages shouldn't be indexed
        { property: "og:title", content: title },
        { property: "og:description", content: description },
        { property: "og:type", content: "website" },
        { name: "twitter:title", content: title },
        { name: "twitter:description", content: description },
      ],
    };
  },
  component: SearchPage,
});

function SearchPage() {
  const navigate = useNavigate();
  const { query = "", page = 1 } = Route.useSearch();

  // Redirect to home if no query
  useEffect(() => {
    if (!query.trim()) {
      navigate({ to: "/" });
    }
  }, [query, navigate]);

  const client = new PaksClient();

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["searchPaks", query, page],
    queryFn: async () => {
      if (!query.trim()) {
        // If no query, list all paks (trending)
        const response = await client.listPaks({
          sort_by: "TRENDING",
          limit: RESULTS_PER_PAGE,
          offset: (page - 1) * RESULTS_PER_PAGE,
        });
        return { items: response.items, total_count: response.total_count };
      }
      // Search with query - returns Pak[] directly
      const results = await client.searchPaks({
        query: query.trim(),
        limit: RESULTS_PER_PAGE,
        offset: (page - 1) * RESULTS_PER_PAGE,
      });
      // Estimate total for search (API doesn't return total count for search)
      const hasMore = results.length === RESULTS_PER_PAGE;
      return { items: results, total_count: hasMore ? (page * RESULTS_PER_PAGE) + 1 : ((page - 1) * RESULTS_PER_PAGE) + results.length };
    },
    staleTime: 30000,
  });

  const handlePageChange = (newPage: number) => {
    navigate({
      to: "/search",
      search: { query: query || undefined, page: newPage },
    });
  };

  // Calculate total pages from total_count
  const totalPages = data ? Math.ceil(data.total_count / RESULTS_PER_PAGE) : 1;

  return (
    <div className="min-h-screen flex flex-col">
      <main className="flex-1 py-8 px-4 sm:px-6 lg:px-8">
        <div className="max-w-5xl mx-auto">
          {/* Header */}
          <div className="mb-8">
            <div className="flex items-center gap-3 mb-2">
              <div className="p-2 bg-primary/10 text-primary border border-primary/20">
                <Search className="w-5 h-5" />
              </div>
              <h1 className="text-2xl font-bold text-foreground">
                {query ? (
                  <>
                    Results for "<span className="text-primary">{query}</span>"
                  </>
                ) : (
                  "Browse Packages"
                )}
              </h1>
            </div>
            {data && (
              <p className="text-sm text-muted-foreground ml-12">
                {data.items.length === 0
                  ? "No packages found"
                  : `Showing ${data.items.length} of ${data.total_count} package${data.total_count !== 1 ? "s" : ""}`}
              </p>
            )}
          </div>

          {/* Loading State */}
          {isLoading && (
            <div className="flex flex-col items-center justify-center py-20">
              <Loader2 className="w-8 h-8 text-primary animate-spin mb-4" />
              <p className="text-muted-foreground">Searching packages...</p>
            </div>
          )}

          {/* Error State */}
          {isError && (
            <div className="flex flex-col items-center justify-center py-20 glass p-8">
              <AlertCircle className="w-10 h-10 text-destructive mb-4" />
              <h2 className="text-lg font-semibold text-foreground mb-2">Something went wrong</h2>
              <p className="text-sm text-muted-foreground text-center max-w-md">
                {error instanceof Error ? error.message : "Failed to load packages. Please try again."}
              </p>
            </div>
          )}

          {/* Empty State */}
          {!isLoading && !isError && data && data.items.length === 0 && (
            <div className="flex flex-col items-center justify-center py-20 glass p-8">
              <Package className="w-12 h-12 text-muted-foreground mb-4" />
              <h2 className="text-lg font-semibold text-foreground mb-2">No packages found</h2>
              <p className="text-sm text-muted-foreground text-center max-w-md">
                {query
                  ? `No packages match "${query}". Try a different search term.`
                  : "No packages available yet."}
              </p>
            </div>
          )}

          {/* Results Grid */}
          {!isLoading && !isError && data && data.items.length > 0 && (
            <>
              <div className="grid gap-4">
                {data.items.map((pak) => (
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
