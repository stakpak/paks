import { createFileRoute } from "@tanstack/react-router";
import { Hero } from "@/components/hero";
import { Features } from "@/components/features";
import { Footer } from "@/components/footer";

export const Route = createFileRoute("/")({ component: App });

function App() {
  return (
    <>
      <Hero />
      <Features />
      <Footer />
    </>
  );
}