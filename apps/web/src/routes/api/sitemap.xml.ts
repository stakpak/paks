import { createFileRoute } from "@tanstack/react-router";
import { PaksClient } from "@paks/api";

const SITE_URL = "https://paks.stakpak.dev";

export const Route = createFileRoute("/api/sitemap/xml")({
  server: {
    handlers: {
      GET: async () => {
        const client = new PaksClient();

        // Fetch all paks for dynamic URLs
        let allPaks: Array<{ owner_name: string; name: string; updated_at?: string }> = [];
        try {
          allPaks = await client.listPaks({ limit: 100, sort_by: "TRENDING" });
        } catch (error) {
          console.error("Failed to fetch paks for sitemap:", error);
        }

        const today = new Date().toISOString().split("T")[0];

        // Static pages
        const staticPages = [
          { url: "/", priority: "1.0", changefreq: "daily" },
          { url: "/trending", priority: "0.9", changefreq: "daily" },
        ];

        // Generate XML
        const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
        http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">
  <!-- Static Pages -->
${staticPages
  .map(
    (page) => `  <url>
    <loc>${SITE_URL}${page.url}</loc>
    <lastmod>${today}</lastmod>
    <changefreq>${page.changefreq}</changefreq>
    <priority>${page.priority}</priority>
  </url>`
  )
  .join("\n")}
  
  <!-- Dynamic Pak Pages -->
${allPaks
  .map(
    (pak) => `  <url>
    <loc>${SITE_URL}/pak/${pak.owner_name}/${pak.name}</loc>
    <lastmod>${pak.updated_at ? new Date(pak.updated_at).toISOString().split("T")[0] : today}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>`
  )
  .join("\n")}
</urlset>`;

        return new Response(xml, {
          headers: {
            "Content-Type": "application/xml",
            "Cache-Control": "public, max-age=3600, s-maxage=3600",
          },
        });
      },
    },
  },
});
