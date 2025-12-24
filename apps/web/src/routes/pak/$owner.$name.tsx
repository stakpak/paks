import { useState } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { PaksClient } from "@paks/api";
import type { PakWithLatestVersion, PakContent } from "@paks/api";
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
  Loader2, 
  AlertCircle,
  Package
} from "lucide-react";

type TabType = "skill.md" | "files" | "versions";

export const Route = createFileRoute("/pak/$owner/$name")({
  component: PakDetailPage,
});

function PakDetailPage() {
  const { owner, name } = Route.useParams();
  const [activeTab, setActiveTab] = useState<TabType>("skill.md");
  
  const client = new PaksClient();

  // Fetch pak details
  const { 
    data: pakData, 
    isLoading: pakLoading, 
    isError: pakError,
    error: pakErrorDetails 
  } = useQuery({
    queryKey: ["pak", owner, name],
    queryFn: async () => {
      const results = await client.searchPaks({
        owner,
        pak_name: name,
        limit: 1,
      });
      if (results.length === 0) {
        throw new Error("Pak not found");
      }
      // Get pak with latest version info using listPaks
      const listResults = await client.listPaks({
        limit: 100,
      });
      const pakWithVersion = listResults.find(
        (p) => p.owner_name === owner && p.name === name
      );
      return pakWithVersion || results[0];
    },
    staleTime: 60000,
  });

  // Fetch pak content (README and files) using full_uri from pak data
  const { data: contentData, isLoading: contentLoading } = useQuery({
    queryKey: ["pakContent", pakData?.full_uri],
    queryFn: () => client.getPakContent(encodeURIComponent(pakData!.full_uri)),
    enabled: !!pakData?.full_uri,
    staleTime: 60000,
  });

  // Get README content from pak content
  const readmeContent = extractReadme(contentData?.content);

  const tabs: { id: TabType; label: string; icon: React.ReactNode }[] = [
    { id: "skill.md", label: "Skill.md", icon: <FileText className="w-4 h-4" /> },
    { id: "files", label: "Files", icon: <FolderTree className="w-4 h-4" /> },
    { id: "versions", label: "Versions", icon: <Tag className="w-4 h-4" /> },
  ];

  // Loading state
  if (pakLoading) {
    return (
      <div className="min-h-screen flex flex-col">
        <main className="flex-1 flex items-center justify-center">
          <div className="flex flex-col items-center gap-4">
            <Loader2 className="w-10 h-10 text-primary animate-spin" />
            <p className="text-muted-foreground">Loading package...</p>
          </div>
        </main>
        <Footer />
      </div>
    );
  }

  // Error state
  if (pakError) {
    return (
      <div className="min-h-screen flex flex-col">
        <main className="flex-1 flex items-center justify-center px-4">
          <div className="flex flex-col items-center gap-4 glass p-8 max-w-md text-center">
            <AlertCircle className="w-12 h-12 text-destructive" />
            <h1 className="text-xl font-bold text-foreground">Package Not Found</h1>
            <p className="text-muted-foreground">
              {pakErrorDetails instanceof Error 
                ? pakErrorDetails.message 
                : `The package "${owner}/${name}" could not be found.`}
            </p>
            <a 
              href="/" 
              className="mt-4 px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
            >
              Go Home
            </a>
          </div>
        </main>
        <Footer />
      </div>
    );
  }

  const pak = pakData as PakWithLatestVersion;
  const latestVersion = "latest_version" in pak ? pak.latest_version : null;

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
                <span className="px-2 py-1 text-xs bg-primary/10 text-primary border border-primary/20 font-medium">
                  {latestVersion.version}
                </span>
              )}
            </div>

            {/* Description */}
            {pak.description && (
              <p className="text-muted-foreground max-w-3xl mb-4">
                {pak.description}
              </p>
            )}

            {/* Tags */}
            {pak.tags && pak.tags.length > 0 && (
              <div className="flex flex-wrap gap-2">
                {pak.tags.map((tag) => (
                  <a
                    key={tag}
                    href={`/search?query=${encodeURIComponent(tag)}`}
                    className="px-2 py-0.5 text-xs bg-accent/50 text-accent-foreground border border-border/30 hover:border-primary/40 hover:bg-primary/10 transition-colors"
                  >
                    {tag}
                  </a>
                ))}
              </div>
            )}
          </header>


          {/* Tabs */}
          <div className="flex gap-1 mb-6 border-b border-border/30">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`
                  relative flex items-center gap-2 px-4 py-3 text-sm font-medium transition-colors
                  ${activeTab === tab.id 
                    ? "text-primary" 
                    : "text-muted-foreground hover:text-foreground"}
                `}
              >
                {tab.icon}
                {tab.label}
                {activeTab === tab.id && (
                  <span className="absolute bottom-0 left-0 right-0 h-[2px] bg-primary glow-cyan-sm" />
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
                  content={readmeContent} 
                  isLoading={contentLoading}
                />
              )}
              {activeTab === "files" && (
                <FileExplorer 
                  content={contentData?.content}
                  isLoading={contentLoading}
                  pakUri={`${owner}/${name}`}
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
            <aside className="lg:sticky lg:top-24 lg:self-start space-y-4">
              {/* Install Command - Most Prominent */}
              <InstallCommand uri={`${owner}/${name}`} />
              
              {/* Metadata */}
              <PakSidebar pak={pak} latestVersion={latestVersion} />
            </aside>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}

// Helper to extract README content from pak content
function extractReadme(content?: PakContent): string | null {
  if (!content) return null;
  
  if (content.type === "File") {
    return content.content;
  }
  
  if (content.type === "Directory") {
    // Look for README file
    const readmeFile = content.items.find((item) => 
      item.type === "file" && 
      item.name.toLowerCase().startsWith("readme")
    );
    if (readmeFile?.content) {
      return readmeFile.content;
    }
  }
  
  return null;
}
