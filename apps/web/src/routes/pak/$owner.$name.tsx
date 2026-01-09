import { useState } from "react";
import { createFileRoute, useLoaderData } from "@tanstack/react-router";
import { PaksClient } from "@paks/api";
import type { PakWithLatestVersion, PakContentResponse } from "@paks/api";
import { InstallCommand } from "@/components/install-command";
import { PakSidebar } from "@/components/pak-sidebar";
import { SkillViewer } from "@/components/readme-viewer";
import { FileExplorer } from "@/components/file-explorer";
import { VersionList } from "@/components/version-list";
import { Footer } from "@/components/footer";
import { 
  FileText, 
  FolderTree, 
  Tag, 
  AlertCircle,
  Package
} from "lucide-react";
import { Link } from "@tanstack/react-router";

type TabType = "skill.md" | "files" | "versions";

// Helper to extract frontmatter from markdown
function parseFrontmatter(content: string): { frontmatter: Record<string, unknown>; body: string } {
  const frontmatterRegex = /^---\s*\n([\s\S]*?)\n---\s*\n?/;
  const match = content.match(frontmatterRegex);
  
  if (!match) {
    return { frontmatter: {}, body: content };
  }
  
  const frontmatterStr = match[1];
  const body = content.replace(frontmatterRegex, '').trim();
  
  // Simple YAML parser for frontmatter
  const frontmatter: Record<string, unknown> = {};
  const lines = frontmatterStr.split('\n');
  let currentKey = '';
  let isMultiline = false;
  let multilineValue = '';
  
  for (const line of lines) {
    if (isMultiline) {
      if (line.startsWith('  ')) {
        multilineValue += line.trim() + ' ';
        continue;
      } else {
        frontmatter[currentKey] = multilineValue.trim();
        isMultiline = false;
      }
    }
    
    const colonIndex = line.indexOf(':');
    if (colonIndex > 0 && !line.startsWith(' ') && !line.startsWith('-')) {
      currentKey = line.slice(0, colonIndex).trim();
      const value = line.slice(colonIndex + 1).trim();
      
      if (value === '' || value === '|') {
        isMultiline = true;
        multilineValue = '';
      } else if (value.startsWith('[') || value.startsWith('-')) {
        // Array - skip for now
        frontmatter[currentKey] = [];
      } else {
        frontmatter[currentKey] = value.replace(/^["']|["']$/g, '');
      }
    }
  }
  
  if (isMultiline) {
    frontmatter[currentKey] = multilineValue.trim();
  }
  
  return { frontmatter, body };
}

// Loader data type
interface PakLoaderData {
  pak: PakWithLatestVersion | null;
  skillContent: string | null;
  filesContent: PakContentResponse | null;
  error?: string;
}

export const Route = createFileRoute("/pak/$owner/$name")({
  // Server-side loader to pre-fetch all data
  loader: async ({ params }): Promise<PakLoaderData> => {
    const client = new PaksClient();
    const { owner, name } = params;
    
    try {
      // First try to fetch SKILL.md content - if this fails, pak doesn't exist
      let skillContent: string | null = null;
      let filesContent: PakContentResponse | null = null;
      
      try {
        const skillData = await client.getPakContent(`${owner}/${name}/SKILL.md`);
        if (skillData?.content?.type === "File" && skillData.content.content) {
          skillContent = skillData.content.content;
        }
      } catch {
        // SKILL.md not found
      }
      
      // Fetch files content  
      try {
        filesContent = await client.getPakContent(`${owner}/${name}`);
      } catch {
        // Files not found
      }
      
      // Fetch pak details from search
      let pakWithVersion: PakWithLatestVersion | null = null;
      try {
        const results = await client.searchPaks({
          owner,
          pak_name: name,
          limit: 1,
        });
        
        if (results.length > 0) {
          // Get pak with latest version info
          const listResponse = await client.listPaks({ limit: 100 });
          pakWithVersion = listResponse.items.find(
            (p) => p.owner_name === owner && p.name === name
          ) || results[0] as PakWithLatestVersion;
        }
      } catch {
        // Search failed, that's okay if we have content
      }
      
      // If no pak found from search but we have content, create a minimal pak object
      if (!pakWithVersion && (skillContent || filesContent)) {
        pakWithVersion = {
          id: '',
          owner_name: owner,
          name: name,
          full_uri: `stakpak://${owner}/${name}`,
          description: null,
          latest_version: null,
          total_downloads: 0,
          download_count: 0,
          version_count: 1,
          tags: [],
          repository_url: null,
          documentation_url: null,
          homepage_url: null,
          license: null,
          updated_at: new Date().toISOString(),
          created_at: new Date().toISOString(),
        } as unknown as PakWithLatestVersion;
      }
      
      if (!pakWithVersion) {
        return { pak: null, skillContent: null, filesContent: null, error: "Pak not found" };
      }
      
      return { pak: pakWithVersion, skillContent, filesContent };
    } catch (err) {
      console.error("Loader error:", err);
      return { 
        pak: null, 
        skillContent: null, 
        filesContent: null, 
        error: err instanceof Error ? err.message : "Failed to load package" 
      };
    }
  },
  
  // SEO meta tags based on pak data
  head: ({ loaderData }) => {
    if (!loaderData) {
      return {
        meta: [
          { title: "Loading... | Paks" },
        ],
      };
    }
    
    const { pak, skillContent } = loaderData;
    
    if (!pak) {
      return {
        meta: [
          { title: "Package Not Found | Paks" },
          { name: "description", content: "The requested package could not be found." },
        ],
      };
    }
    
    // Parse frontmatter from SKILL.md for additional metadata
    let frontmatter: Record<string, unknown> = {};
    if (skillContent) {
      const parsed = parseFrontmatter(skillContent);
      frontmatter = parsed.frontmatter;
    }
    
    // Build title and description
    const title = `${pak.owner_name}/${pak.name} | Paks`;
    const description = pak.description 
      || (frontmatter.description as string)
      || `${pak.name} - AI Agent Skill package for coding agents`;
    const keywords = (pak.tags && pak.tags.length > 0)
      ? pak.tags.join(", ")
      : (frontmatter.tags as string[] || []).join(", ");
    const version = pak.latest_version?.version || "1.0.0";
    const author = (frontmatter.metadata as Record<string, string>)?.author 
      || pak.owner_name;
    
    return {
      meta: [
        { title },
        { name: "description", content: description },
        { name: "keywords", content: keywords },
        { name: "author", content: author },
        // Open Graph
        { property: "og:title", content: title },
        { property: "og:description", content: description },
        { property: "og:type", content: "website" },
        { property: "og:url", content: `https://paks.dev/pak/${pak.owner_name}/${pak.name}` },
        { property: "og:image", content: `https://paks.stakpak.dev/api/og/${pak.owner_name}/${pak.name}/png` },
        // Twitter
        { name: "twitter:card", content: "summary_large_image" },
        { name: "twitter:title", content: title },
        { name: "twitter:description", content: description },
        { name: "twitter:image", content: `https://paks.stakpak.dev/api/og/${pak.owner_name}/${pak.name}/png` },
        // Package specific
        { name: "pak:name", content: `${pak.owner_name}/${pak.name}` },
        { name: "pak:version", content: version },
        { name: "pak:downloads", content: String(pak.total_downloads) },
      ],
    };
  },
  
  component: PakDetailPage,
  
  // Error boundary to catch rendering errors
  errorComponent: ({ error }) => (
    <div className="min-h-screen flex flex-col">
      <main className="flex-1 flex items-center justify-center px-4">
        <div className="flex flex-col items-center gap-4 glass p-8 max-w-md text-center">
          <AlertCircle className="w-12 h-12 text-destructive" />
          <h1 className="text-xl font-bold text-foreground">Something went wrong</h1>
          <p className="text-muted-foreground">
            {error instanceof Error ? error.message : "An unexpected error occurred while loading this package."}
          </p>
          <Link 
            to="/" 
            className="mt-4 px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
          >
            Go Home
          </Link>
        </div>
      </main>
      <Footer />
    </div>
  ),
});

function PakDetailPage() {
  const { owner, name } = Route.useParams();
  const { pak, skillContent, filesContent, error } = useLoaderData({ from: "/pak/$owner/$name" });
  const [activeTab, setActiveTab] = useState<TabType>("skill.md");

  const tabs: { id: TabType; label: string; icon: React.ReactNode }[] = [
    { id: "skill.md", label: "Skill.md", icon: <FileText className="w-4 h-4" /> },
    { id: "files", label: "Files", icon: <FolderTree className="w-4 h-4" /> },
    { id: "versions", label: "Versions", icon: <Tag className="w-4 h-4" /> },
  ];

  // Error state
  if (error || !pak) {
    return (
      <div className="min-h-screen flex flex-col">
        <main className="flex-1 flex items-center justify-center px-4">
          <div className="flex flex-col items-center gap-4 glass p-8 max-w-md text-center">
            <AlertCircle className="w-12 h-12 text-destructive" />
            <h1 className="text-xl font-bold text-foreground">Package Not Found</h1>
            <p className="text-muted-foreground">
              {error || `The package "${owner}/${name}" could not be found.`}
            </p>
            <Link 
              to="/" 
              className="mt-4 px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
            >
              Go Home
            </Link>
          </div>
        </main>
        <Footer />
      </div>
    );
  }

  const latestVersion = pak.latest_version;

  return (
    <div className="min-h-screen flex flex-col">
      <main className="flex-1 py-8 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto">
          {/* Header */}
          <header className="mb-8">
            {/* Package Title Row */}
            <div className="flex flex-wrap items-center gap-3 mb-4">
              <div className="p-2.5 bg-primary/10 border border-primary/20">
                <Package className="w-6 h-6 text-primary" />
              </div>
              <div className="flex items-baseline gap-2">
                <span className="text-muted-foreground text-lg">{owner}/</span>
                <h1 className="text-2xl sm:text-3xl font-bold gradient-text">{name}</h1>
              </div>
              {latestVersion && (
                <span className="px-2 py-1 text-xs bg-[oklch(0.75_0.16_55/10%)] text-[oklch(0.75_0.16_55)] border border-[oklch(0.75_0.16_55/20%)] font-medium">
                  {latestVersion.version}
                </span>
              )}
            </div>

            {/* Description */}
            {pak.description && (
              <p className="text-muted-foreground text-base mb-4 max-w-3xl">
                {pak.description}
              </p>
            )}

            {/* Tags */}
            {pak.tags && pak.tags.length > 0 && (
              <div className="flex flex-wrap gap-2">
                {pak.tags.map((tag) => (
                  <Link
                    key={tag}
                    to="/search"
                    search={{ query: tag }}
                    className="px-2.5 py-1 text-xs bg-muted/50 text-muted-foreground hover:text-primary hover:bg-primary/10 border border-border/30 hover:border-primary/30 transition-colors"
                  >
                    {tag}
                  </Link>
                ))}
              </div>
            )}

            {/* Install Command - Full Width */}
            <div className="mt-6">
              <InstallCommand uri={`${owner}/${name}`} />
            </div>
          </header>

          {/* Tab Navigation */}
          <div className="flex items-center gap-1 mb-6 border-b border-border/30">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`
                  flex items-center gap-2 px-4 py-3 text-sm font-medium transition-colors relative
                  ${activeTab === tab.id 
                    ? "text-primary" 
                    : "text-muted-foreground hover:text-foreground"}
                `}
              >
                {tab.icon}
                {tab.label}
                {activeTab === tab.id && (
                  <span className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />
                )}
              </button>
            ))}
          </div>

          {/* Content Layout: Main + Sidebar (70/30 split) */}
          <div className="grid grid-cols-1 lg:grid-cols-[7fr_3fr] gap-8">
            {/* Main Content */}
            <div className="min-w-0">
              {activeTab === "skill.md" && (
                <SkillViewer 
                  content={skillContent} 
                  isLoading={false}
                />
              )}
              {activeTab === "files" && (
                <FileExplorer 
                  content={filesContent?.content}
                  isLoading={false}
                  pakUri={pak.uri || `${owner}/${name}`}
                  skillContent={skillContent}
                />
              )}
              {activeTab === "versions" && (
                <VersionList 
                  pak={pak}
                  latestVersion={latestVersion}
                />
              )}
            </div>

            {/* Sidebar */}
            <aside className="lg:sticky lg:top-24 lg:self-start space-y-4 min-w-0 overflow-hidden">
              {/* Package Info */}
              <PakSidebar pak={pak} latestVersion={latestVersion} />
            </aside>
          </div>
        </div>
      </main>
      <Footer />
    </div>
  );
}
