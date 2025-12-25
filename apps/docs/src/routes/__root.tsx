import { createRootRoute, HeadContent, Outlet, Scripts } from "@tanstack/react-router";
import * as React from "react";
import appCss from "@/styles/app.css?url";
import { RootProvider } from "fumadocs-ui/provider/tanstack";

const SITE_URL = "https://paks.stakpak.dev/docs";
const SITE_NAME = "Paks Docs";
const DEFAULT_TITLE = "Paks Documentation - AI Agent Skills Package Manager";
const DEFAULT_DESCRIPTION = "Complete documentation for Paks CLI - create, install, publish, and manage skills for AI coding agents like Claude Code, Cursor, and GitHub Copilot.";

export const Route = createRootRoute({
  head: () => ({
    meta: [
      { charSet: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { title: DEFAULT_TITLE },
      { name: "description", content: DEFAULT_DESCRIPTION },
      { name: "keywords", content: "paks, cli, ai agents, skills, package manager, claude code, cursor, copilot, documentation" },
      { name: "author", content: "Stakpak" },
      { name: "robots", content: "index, follow" },
      
      // Open Graph
      { property: "og:type", content: "website" },
      { property: "og:site_name", content: SITE_NAME },
      { property: "og:title", content: DEFAULT_TITLE },
      { property: "og:description", content: DEFAULT_DESCRIPTION },
      { property: "og:url", content: SITE_URL },
      
      // Twitter
      { name: "twitter:card", content: "summary_large_image" },
      { name: "twitter:site", content: "@stakpak" },
      { name: "twitter:title", content: DEFAULT_TITLE },
      { name: "twitter:description", content: DEFAULT_DESCRIPTION },
      
      // Theme
      { name: "theme-color", content: "#0d1117" },
      { name: "color-scheme", content: "dark" },
    ],
    links: [
      { rel: "stylesheet", href: appCss },
      { rel: "icon", href: "https://paks.stakpak.dev/icons/favicon.ico" },
    ],
  }),
  component: RootComponent,
});

function RootComponent() {
  return (
    <RootDocument>
      <Outlet />
    </RootDocument>
  );
}

function RootDocument({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark" suppressHydrationWarning>
      <head>
        <HeadContent />
      </head>
      <body className="flex flex-col min-h-screen">
        <RootProvider>{children}</RootProvider>
        <Scripts />
      </body>
    </html>
  );
}
