import { createFileRoute } from "@tanstack/react-router";
import { PaksClient } from "@paks/api";
import satori from "satori";
import { Resvg } from "@resvg/resvg-js";

// Brand colors from app - extracted from index.css
// Background: oklch(0.12 0.01 250) ≈ #040609
// Logo gradient: #049be7 (blue) → #0ce4a8 (teal) from logo.svg
// Foreground: oklch(0.95 0.01 200) ≈ #e8eaed
// Muted: oklch(0.65 0.02 200) ≈ #94a3b8

// Cache fonts in memory to avoid refetching
let fontsCache: Array<{ name: string; data: ArrayBuffer; weight: 400 | 700; style: "normal" }> | null = null;

import { readFile } from "fs/promises";
import { join } from "path";

// Fetch JetBrains Mono font from local file system
async function loadFonts() {
  if (fontsCache) return fontsCache;
  
  try {
    let fontPath = join(process.cwd(), "apps/web/public/fonts");
    // If running from apps/web directly (e.g. dev server)
    if (process.cwd().endsWith("web")) {
      fontPath = join(process.cwd(), "public/fonts");
    }
    
    // Read fonts from local public directory
    const [jetbrainsRegular, jetbrainsBold] = await Promise.all([
      readFile(join(fontPath, "JetBrainsMono-Regular.ttf")),
      readFile(join(fontPath, "JetBrainsMono-Bold.ttf")),
    ]);

    fontsCache = [
      {
        name: "JetBrains Mono",
        data: jetbrainsRegular.buffer as ArrayBuffer,
        weight: 400 as const,
        style: "normal" as const,
      },
      {
        name: "JetBrains Mono",
        data: jetbrainsBold.buffer as ArrayBuffer,
        weight: 700 as const,
        style: "normal" as const,
      },
    ];
    
    return fontsCache;
  } catch (error) {
    console.warn("Failed to load local fonts, falling back to fetch:", error);
    // Fallback if local read fails (e.g. in certain deployment environments where public dir isn't adjacent)
    const jetbrainsRegular = await fetch(
      "https://raw.githubusercontent.com/JetBrains/JetBrainsMono/master/fonts/ttf/JetBrainsMono-Regular.ttf"
    ).then((res) => res.arrayBuffer());

    const jetbrainsBold = await fetch(
      "https://raw.githubusercontent.com/JetBrains/JetBrainsMono/master/fonts/ttf/JetBrainsMono-Bold.ttf"
    ).then((res) => res.arrayBuffer());

    fontsCache = [
      {
        name: "JetBrains Mono",
        data: jetbrainsRegular,
        weight: 400 as const,
        style: "normal" as const,
      },
      {
        name: "JetBrains Mono",
        data: jetbrainsBold,
        weight: 700 as const,
        style: "normal" as const,
      },
    ];
    return fontsCache;
  }
}

// Format numbers with K/M suffixes
function formatNumber(num: number): string {
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
  return num.toString();
}

// Format description: try to take the first sentence, fallback to truncation
function formatDescription(text: string, maxLength: number = 200): string {
  // Try splitting by the first sentence
  const sentences = text.split(".");
  const firstSentence = sentences[0];

  // If the first sentence is short enough, use it (add dot back if it was stripped)
  if (firstSentence && firstSentence.length <= maxLength && sentences.length > 1) {
    return firstSentence + ".";
  }
  
  // Also if the whole text is short enough, just return it
  if (text.length <= maxLength) {
    return text;
  }

  // Fallback: standard truncation respect word boundaries
  const subString = text.slice(0, maxLength);
  const lastSpaceIndex = subString.lastIndexOf(" ");
  
  if (lastSpaceIndex === -1) {
    return subString + "...";
  }
  
  return subString.slice(0, lastSpaceIndex) + "...";
}

// Paks Logo SVG as React component (simplified paths from logo.svg)
function PaksLogo({ size = 56 }: { size?: number }) {
  return (
    <svg
      width={size}
      height={size * (412.783 / 387.89)}
      viewBox="0 0 387.89 412.783"
      style={{ display: "flex" }}
    >
      <defs>
        <linearGradient id="logoGrad" x1="1" y1="0.851" x2="0" y2="0.244">
          <stop offset="0" stopColor="#049be7" />
          <stop offset="1" stopColor="#0ce4a8" />
        </linearGradient>
      </defs>
      <g transform="translate(-300 -286)">
        <path
          d="M478,292.3c-7.4,3.5-26.3,12.9-42,20.9s-43.8,22.3-62.5,31.8c-37.8,19.3-41.5,21.4-41.5,23.5,0,.9,9.7,6.3,26.3,14.6,14.4,7.2,26.7,13.5,27.2,13.9.6.4,25,12.8,54.2,27.4,41.2,20.7,53.8,26.6,55.5,26.1,2.3-.7,12.4-5.8,87.8-44.6,71.7-36.8,70.5-36.2,70.5-38.4,0-1.6-13.9-9-79.3-41.8-43.5-21.8-80-39.7-81-39.7-.9,0-7.8,2.8-15.2,6.3M300.7,388.7c-.4.3-.7,19.2-.7,41.8,0,34.8.2,41.9,1.6,45.8l1.5,4.6,35.2,18.2c56.5,29.2,63.8,32.9,92.2,47.4,14.9,7.6,31.6,16.2,37.3,19.2,5.6,3,10.7,5.1,11.2,4.8,1.2-.7,1.4-90.4.2-92.1-.7-1.1-38.3-20.4-106.7-54.7-20.3-10.2-44.4-22.4-53.5-27.1-17.2-9-17.3-9-18.3-7.9m365.8,7.9c-9.3,4.7-43.8,22.4-76.5,39.2-32.7,16.9-63.9,32.9-69.2,35.7-9,4.6-9.8,5.3-10.5,8.5-.7,4.1.4,87.7,1.3,89.1.5.8,46.4-21.7,64.4-31.7,4.1-2.2,15.4-8.1,25-12.9,9.6-4.9,20.9-10.7,25-13s17-9,28.5-15c11.6-5.9,23.1-12.1,25.7-13.7,7.9-5,7.9-4.8,7.6-52.8-.3-40.7-.3-41.5-2.3-41.7-1.1-.1-9.6,3.6-19,8.3M300.4,514.3c-.3.8-.4,18.9-.2,40.3.4,46.1,0,44.6,10.9,50.3,3.5,1.8,10.7,5.7,15.9,8.6,16.6,9.2,66,34.5,67.5,34.5,1.2,0,1.5-2.3,1.7-12.3.4-22.6-.3-74.2-1-74.8-1.8-1.7-91.8-47.9-93.3-47.9a2.09,2.09,0,0,0-1.5,1.3"
          fill="url(#logoGrad)"
        />
        <path
          d="M653.5,528.8c-17,8.7-43.8,22.5-59.5,30.6-64,33.4-80,41.6-89.8,46.5L494,611l-29.3-14.6c-16-8-30.7-15.5-32.4-16.6-1.8-1.1-3.7-1.7-4.3-1.3-1.2.7-1.4,84.3-.2,86.1.7,1.1,37.6,21.1,55.1,29.9,7.2,3.6,10.2,4.6,12.6,4.2,4.2-.7,13.1-5,38-18.2,11.6-6.2,31.1-16.4,43.5-22.7,38.5-19.8,51.1-26.3,64.5-33.5,7.2-3.8,19-10,26.3-13.8,14.1-7.2,18.7-11.1,19.7-16.7.3-1.8.4-20.6.3-41.8-.3-33.3-.5-38.5-1.8-38.7-.8-.2-15.4,6.8-32.5,15.5"
          fill="url(#logoGrad)"
        />
      </g>
    </svg>
  );
}

// OG Image component with exact Paks brand styling
function OGImageTemplate({
  pakName,
  pakOwner,
  description,
  visibility,
  downloads,
}: {
  pakName: string;
  pakOwner: string;
  description: string;
  visibility: string;
  downloads: number;
}) {
  return (
    <div
      style={{
        height: "100%",
        width: "100%",
        display: "flex",
        flexDirection: "column",
        padding: "60px",
        // Exact background from CSS: oklch(0.12 0.01 250) = #040609
        background: "#040609",
        fontFamily: "JetBrains Mono",
      }}
    >
      {/* Header with actual Paks logo + gradient "Paks" text */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          marginBottom: "48px",
        }}
      >
        {/* Actual Paks logo from logo.svg */}
        <PaksLogo size={52} />
        {/* "Paks" text with logo gradient */}
        <span
          style={{
            marginLeft: "16px",
            fontSize: "36px",
            fontWeight: 700,
            // Gradient matching logo.svg: #049be7 → #0ce4a8
            background: "linear-gradient(135deg, #049be7 0%, #0ce4a8 100%)",
            backgroundClip: "text",
            color: "transparent",
          }}
        >
          Paks
        </span>
      </div>

      {/* Main content */}
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 1,
        }}
      >
        {/* Owner name */}
        <div
          style={{
            display: "flex",
            fontSize: "24px",
            // Muted foreground
            color: "#94a3b8",
            marginBottom: "8px",
          }}
        >
          {pakOwner}/
        </div>
        {/* Package name with logo gradient */}
        <div
          style={{
            display: "flex",
            fontSize: "56px",
            fontWeight: 700,
            // Logo gradient for pak name
            background: "linear-gradient(90deg, #049be7 0%, #0ce4a8 100%)",
            backgroundClip: "text",
            color: "transparent",
            marginBottom: "20px",
            lineHeight: 1.1,
          }}
        >
          {pakName}
        </div>
        {/* Description */}
        <div
          style={{
            display: "flex",
            fontSize: "22px",
            // Foreground: oklch(0.95 0.01 200) ≈ #e8eaed
            color: "#e8eaed",
            lineHeight: 1.5,
            maxWidth: "950px",
          }}
        >
          {formatDescription(description, 200)}
        </div>
      </div>

      {/* Footer with stats */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "24px",
          marginTop: "auto",
        }}
      >
        {/* Visibility badge - using logo colors */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            padding: "8px 16px",
            // Teal accent with transparency
            background: "rgba(12, 228, 168, 0.15)",
            border: "1px solid rgba(12, 228, 168, 0.4)",
            borderRadius: "4px",
          }}
        >
          <span
            style={{
              fontSize: "16px",
              color: "#0ce4a8",
              fontWeight: 600,
              textTransform: "uppercase",
              letterSpacing: "0.05em",
            }}
          >
            {visibility}
          </span>
        </div>
        {/* Downloads */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "8px",
          }}
        >
          <span
            style={{
              fontSize: "18px",
              color: "#94a3b8",
            }}
          >
            ↓
          </span>
          <span
            style={{
              fontSize: "18px",
              color: "#94a3b8",
            }}
          >
            {formatNumber(downloads)} downloads
          </span>
        </div>
      </div>
    </div>
  );
}

export const Route = createFileRoute("/api/og/$owner/$name/png")({
  server: {
    handlers: {
      GET: async ({ params }) => {
        const { owner, name } = params;
        const client = new PaksClient();

        // Fetch pak data
        let pakName = name;
        let pakOwner = owner;
        let description = "AI Agent Skill package for coding agents";
        let visibility = "PUBLIC";
        let downloads = 0;

        try {
          const results = await client.searchPaks({
            owner,
            pak_name: name,
            limit: 1,
          });

          if (results.length > 0) {
            const pak = results[0];
            pakName = pak.name;
            pakOwner = pak.owner_name;
            description = pak.description || description;
            downloads = pak.total_downloads;
            visibility = pak.visibility;
          }
        } catch (error) {
          console.error("Failed to fetch pak data for OG image:", error);
        }

        // Load fonts
        const fonts = await loadFonts();

        // Generate SVG using Satori
        const svg = await satori(
          <OGImageTemplate
            pakName={pakName}
            pakOwner={pakOwner}
            description={description}
            visibility={visibility}
            downloads={downloads}
          />,
          {
            width: 1200,
            height: 630,
            fonts,
          }
        );

        // Convert SVG to PNG
        const resvg = new Resvg(svg, {
          fitTo: {
            mode: "width",
            value: 1200,
          },
        });
        const pngData = resvg.render();
        const pngBuffer = pngData.asPng();

        return new Response(new Uint8Array(pngBuffer), {
          headers: {
            "Content-Type": "image/png",
            "Cache-Control": "public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800",
          },
        });
      },
    },
  },
});
