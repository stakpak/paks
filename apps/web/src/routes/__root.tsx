import { HeadContent, Outlet, Scripts, createRootRouteWithContext } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { Toaster } from "@/components/ui/sonner";
import { Navbar } from "@/components/navbar";

import appCss from "../index.css?url";

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60 * 1000, // 1 minute
      retry: 1,
    },
  },
});

export interface RouterAppContext {}

export const Route = createRootRouteWithContext<RouterAppContext>()({
  head: () => ({
    meta: [
      {
        charSet: "utf-8",
      },
      {
        name: "viewport",
        content: "width=device-width, initial-scale=1",
      },
      {
        title: "Paks - AI Agent Skills Package Manager",
      },
      {
        name: "description",
        content: "Create, install, publish, and share reusable skills for AI coding agents like Claude Code, Cursor, and GitHub Copilot.",
      },
      {
        name: "msapplication-TileColor",
        content: "#ffffff",
      },
      {
        name: "msapplication-TileImage",
        content: "/icons/ms-icon-144x144.png",
      },
      {
        name: "theme-color",
        content: "#0d1117",
      },
    ],
    links: [
      {
        rel: "stylesheet",
        href: appCss,
      },
      // Apple Touch Icons
      { rel: "apple-touch-icon", sizes: "57x57", href: "/icons/apple-icon-57x57.png" },
      { rel: "apple-touch-icon", sizes: "60x60", href: "/icons/apple-icon-60x60.png" },
      { rel: "apple-touch-icon", sizes: "72x72", href: "/icons/apple-icon-72x72.png" },
      { rel: "apple-touch-icon", sizes: "76x76", href: "/icons/apple-icon-76x76.png" },
      { rel: "apple-touch-icon", sizes: "114x114", href: "/icons/apple-icon-114x114.png" },
      { rel: "apple-touch-icon", sizes: "120x120", href: "/icons/apple-icon-120x120.png" },
      { rel: "apple-touch-icon", sizes: "144x144", href: "/icons/apple-icon-144x144.png" },
      { rel: "apple-touch-icon", sizes: "152x152", href: "/icons/apple-icon-152x152.png" },
      { rel: "apple-touch-icon", sizes: "180x180", href: "/icons/apple-icon-180x180.png" },
      // Android and general favicons
      { rel: "icon", type: "image/png", sizes: "192x192", href: "/icons/android-icon-192x192.png" },
      { rel: "icon", type: "image/png", sizes: "32x32", href: "/icons/favicon-32x32.png" },
      { rel: "icon", type: "image/png", sizes: "96x96", href: "/icons/favicon-96x96.png" },
      { rel: "icon", type: "image/png", sizes: "16x16", href: "/icons/favicon-16x16.png" },
      // Manifest
      { rel: "manifest", href: "/icons/manifest.json" },
    ],
  }),

  component: RootDocument,
});

function RootDocument() {
  return (
    <html lang="en" className="dark">
      <head>
        <HeadContent />
      </head>
      <body className="min-h-screen bg-background text-foreground">
        <QueryClientProvider client={queryClient}>
          <Navbar />
          <main className="pt-28 min-[550px]:pt-16">
            <Outlet />
          </main>
          <Toaster richColors />
        </QueryClientProvider>
        <TanStackRouterDevtools position="bottom-left" />
        <Scripts />
      </body>
    </html>
  );
}
