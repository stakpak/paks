import { createFileRoute, useLoaderData } from "@tanstack/react-router";
import { PaksClient } from "@paks/api";
import { Hero } from "@/components/hero";
import { TrendingPaks } from "@/components/trending-paks";
import { Features } from "@/components/features";
import { Footer } from "@/components/footer";

export const Route = createFileRoute("/")({
  loader: async () => {
    const client = new PaksClient();
    const trendingPaks = await client.listPaks({ sort_by: "TRENDING", limit: 6 });
    return { trendingPaks };
  },
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