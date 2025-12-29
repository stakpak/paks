import { createFileRoute, useLoaderData } from "@tanstack/react-router";
import { PaksClient } from "@paks/api";
import { Hero } from "@/components/hero";
import { TrendingPaks } from "@/components/trending-paks";
import { Features } from "@/components/features";
import { Footer } from "@/components/footer";

export const Route = createFileRoute("/")({
  loader: async () => {
    const client = new PaksClient();
    const response = await client.listPaks({ sort_by: "TRENDING", limit: 6 });
    return { trendingPaks: response.items };
  },
  head: () => ({
    meta: [
      { title: "Paks - AI Agent Skills Package Manager" },
      { name: "description", content: "Create, install, publish, and share reusable skills for AI coding agents like Claude Code, Cursor, and GitHub Copilot. The npm for AI agent capabilities." },
      { property: "og:image", content: "https://paks.stakpak.dev/meta-img.jpg" },
      { name: "twitter:image", content: "https://paks.stakpak.dev/meta-img.jpg" },
    ],
    links: [
      { rel: "canonical", href: "https://paks.stakpak.dev/" },
    ],
  }),
  component: App,
});

function App() {
  const { trendingPaks } = useLoaderData({ from: "/" });
  
  return (
    <>
      <Hero />
      <TrendingPaks paks={trendingPaks} />
      <Features />
      <Footer />
    </>
  );
}