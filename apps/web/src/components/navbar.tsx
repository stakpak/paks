import { useState, useEffect, useCallback } from "react";
import { useNavigate, useSearch } from "@tanstack/react-router";
import { Search } from "lucide-react";
import { Input } from "@/components/ui/input";

export function Navbar() {
  const navigate = useNavigate();
  
  // Get query from URL if on search page
  const searchParams = useSearch({ strict: false }) as { query?: string } | undefined;
  const urlQuery = searchParams?.query ?? "";
  
  const [searchValue, setSearchValue] = useState(urlQuery);

  // Sync input with URL query when it changes (e.g., clicking a tag)
  useEffect(() => {
    setSearchValue(urlQuery);
  }, [urlQuery]);

  const handleSearch = useCallback(() => {
    const trimmed = searchValue.trim();
    if (trimmed) {
      navigate({
        to: "/search",
        search: { query: trimmed, page: 1 },
      });
    } else {
      navigate({
        to: "/search",
        search: { page: 1 },
      });
    }
  }, [searchValue, navigate]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleSearch();
    }
  };

  return (
    <nav className="fixed top-0 left-0 right-0 z-50 glass-strong">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        {/* Main row: Logo + Auth (always visible) + Search (hidden on small) */}
        <div className="flex h-16 items-center justify-between gap-4">
          {/* Logo */}
          <a href="/" className="flex-shrink-0 flex items-center gap-2">
            <img src="/logo.svg" alt="Paks" className="w-7 h-7" />
            <span className="text-2xl font-bold gradient-text-animated tracking-tight">
              Paks
            </span>
          </a>

          {/* Search Bar - Always visible beside logo */}
          <div className="flex flex-1 ml-4">
            <div className="relative group w-full">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground transition-colors group-focus-within:text-primary" />
              <Input
                type="search"
                placeholder="Search packages..."
                value={searchValue}
                onChange={(e) => setSearchValue(e.target.value)}
                onKeyDown={handleKeyDown}
                className="w-full pl-10 pr-12 h-10 bg-black/40 border border-white/10 focus:border-primary/60 focus:bg-black/50 transition-all placeholder:text-muted-foreground/70"
              />
              <div className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-1">
                <kbd className="hidden sm:inline-flex h-5 select-none items-center gap-1 border border-border/50 bg-muted/50 px-1.5 text-[10px] font-medium text-muted-foreground">
                  <span className="text-xs">↵</span>
                </kbd>
              </div>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
}
