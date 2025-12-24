import { useEffect } from "react";
import { HeadContent, Outlet, Scripts, createRootRouteWithContext } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import posthog from "posthog-js";
import { PostHogProvider } from "posthog-js/react";

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

const SITE_URL = "https://paks.stakpak.dev";
const SITE_NAME = "Paks";
const DEFAULT_TITLE = "Paks - AI Agent Skills Package Manager";
const DEFAULT_DESCRIPTION = "Create, install, publish, and share reusable skills for AI coding agents like Claude Code, Cursor, and GitHub Copilot. The npm for AI agent capabilities.";
const DEFAULT_KEYWORDS = "AI agents, coding agents, Claude Code, Cursor, GitHub Copilot, package manager, skills, rulebooks, AI tools, developer tools, automation";

export const Route = createRootRouteWithContext<RouterAppContext>()({
  head: () => ({
    meta: [
      // Basic Meta
      { charSet: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { title: DEFAULT_TITLE },
      { name: "description", content: DEFAULT_DESCRIPTION },
      { name: "keywords", content: DEFAULT_KEYWORDS },
      { name: "author", content: "Stakpak" },
      { name: "robots", content: "index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1" },
      { name: "googlebot", content: "index, follow" },
      
      // Open Graph (Facebook, LinkedIn, etc.)
      { property: "og:type", content: "website" },
      { property: "og:site_name", content: SITE_NAME },
      { property: "og:title", content: DEFAULT_TITLE },
      { property: "og:description", content: DEFAULT_DESCRIPTION },
      { property: "og:url", content: SITE_URL },
      { property: "og:image", content: `${SITE_URL}/og-image.png` },
      { property: "og:image:width", content: "1200" },
      { property: "og:image:height", content: "630" },
      { property: "og:image:alt", content: "Paks - AI Agent Skills Package Manager" },
      { property: "og:locale", content: "en_US" },
      
      // Twitter Card
      { name: "twitter:card", content: "summary_large_image" },
      { name: "twitter:site", content: "@stakpak" },
      { name: "twitter:creator", content: "@stakpak" },
      { name: "twitter:title", content: DEFAULT_TITLE },
      { name: "twitter:description", content: DEFAULT_DESCRIPTION },
      { name: "twitter:image", content: `${SITE_URL}/og-image.png` },
      { name: "twitter:image:alt", content: "Paks - AI Agent Skills Package Manager" },
      
      // Microsoft/Windows
      { name: "msapplication-TileColor", content: "#0d1117" },
      { name: "msapplication-TileImage", content: "/icons/ms-icon-144x144.png" },
      { name: "msapplication-config", content: "/icons/browserconfig.xml" },
      
      // Theme
      { name: "theme-color", content: "#0d1117" },
      { name: "color-scheme", content: "dark" },
      
      // App-specific
      { name: "application-name", content: SITE_NAME },
      { name: "apple-mobile-web-app-title", content: SITE_NAME },
      { name: "apple-mobile-web-app-capable", content: "yes" },
      { name: "apple-mobile-web-app-status-bar-style", content: "black-translucent" },
      { name: "mobile-web-app-capable", content: "yes" },
      
      // Verification (add your IDs when available)
      // { name: "google-site-verification", content: "YOUR_GOOGLE_VERIFICATION_ID" },
      // { name: "msvalidate.01", content: "YOUR_BING_VERIFICATION_ID" },
    ],
    links: [
      { rel: "stylesheet", href: appCss },
      
      // Canonical URL
      { rel: "canonical", href: SITE_URL },
      
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
      { rel: "icon", href: "/icons/favicon.ico" },
      
      // Manifest
      { rel: "manifest", href: "/icons/manifest.json" },
      
      // Preconnect for performance
      { rel: "preconnect", href: "https://api.paks.stakpak.dev" },
      { rel: "dns-prefetch", href: "https://api.paks.stakpak.dev" },
    ],
    scripts: [
      // JSON-LD Structured Data for Organization
      {
        type: "application/ld+json",
        children: JSON.stringify({
          "@context": "https://schema.org",
          "@type": "WebApplication",
          "name": "Paks",
          "description": DEFAULT_DESCRIPTION,
          "url": SITE_URL,
          "applicationCategory": "DeveloperApplication",
          "operatingSystem": "Any",
          "offers": {
            "@type": "Offer",
            "price": "0",
            "priceCurrency": "USD"
          },
          "author": {
            "@type": "Organization",
            "name": "Stakpak",
            "url": "https://stakpak.dev"
          },
          "publisher": {
            "@type": "Organization",
            "name": "Stakpak",
            "url": "https://stakpak.dev",
            "logo": {
              "@type": "ImageObject",
              "url": `${SITE_URL}/logo.svg`
            }
          }
        }),
      },
    ],
  }),

  component: RootDocument,
});

function RootDocument() {
  // Initialize PostHog only on paks.dev (production)
  useEffect(() => {
    if (typeof window !== "undefined" && window.location.host === "paks.stakpak.dev") {
      posthog.init("phc_QA5vkh1LnITsEmIhDeSZ2cE8veaBdpUKceWa3b9X3K9", {
        api_host: "https://app.posthog.com",
        capture_pageview: true,
        capture_pageleave: true,
      });
    }
  }, []);

  const isProduction = typeof window !== "undefined" && window.location.host === "paks.stakpak.dev";

  const content = (
    <html lang="en" className="dark">
      <head>
        <HeadContent />
      </head>
      <body className="min-h-screen bg-background text-foreground">
        <QueryClientProvider client={queryClient}>
          <Navbar />
          <main className="pt-16">
            <Outlet />
          </main>
          <Toaster richColors />
        </QueryClientProvider>
        <TanStackRouterDevtools position="bottom-left" />
        <Scripts />
      </body>
    </html>
  );

  // Wrap with PostHog provider only in production
  if (isProduction) {
    return <PostHogProvider client={posthog}>{content}</PostHogProvider>;
  }

  return content;
}
