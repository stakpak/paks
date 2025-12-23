import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { PaksClient } from "@paks/api";
import type { PakContent, ContentItem } from "@paks/api";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";
import { 
  Folder, 
  FolderOpen,
  FileText, 
  FileCode,
  ChevronRight,
  ChevronDown,
  Loader2,
  X
} from "lucide-react";

interface FileExplorerProps {
  content?: PakContent;
  isLoading?: boolean;
  pakUri: string;
}

export function FileExplorer({ content, isLoading, pakUri }: FileExplorerProps) {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [expandedDirs, setExpandedDirs] = useState<Set<string>>(new Set());

  const client = new PaksClient();

  // Fetch file content when selected
  const { data: fileContent, isLoading: fileLoading } = useQuery({
    queryKey: ["fileContent", pakUri, selectedFile],
    queryFn: () => client.getPakContent(`${pakUri}/${selectedFile}`),
    enabled: !!selectedFile,
    staleTime: 60000,
  });

  if (isLoading) {
    return (
      <div className="glass border border-border/30 p-8">
        <div className="flex flex-col items-center justify-center py-12">
          <Loader2 className="w-6 h-6 text-primary animate-spin mb-3" />
          <p className="text-sm text-muted-foreground">Loading files...</p>
        </div>
      </div>
    );
  }

  if (!content || content.type !== "Directory") {
    return (
      <div className="glass border border-border/30 p-8">
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <Folder className="w-10 h-10 text-muted-foreground mb-4" />
          <h3 className="text-lg font-medium text-foreground mb-2">No Files</h3>
          <p className="text-sm text-muted-foreground">
            Unable to load package files.
          </p>
        </div>
      </div>
    );
  }

  const toggleDir = (path: string) => {
    setExpandedDirs((prev) => {
      const next = new Set(prev);
      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }
      return next;
    });
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-[280px,1fr] gap-4">
      {/* File Tree */}
      <div className="glass border border-border/30 p-3 max-h-[600px] overflow-y-auto scrollbar-thin">
        <div className="space-y-0.5">
          {content.items.map((item) => (
            <FileTreeItem
              key={item.uri}
              item={item}
              depth={0}
              expandedDirs={expandedDirs}
              toggleDir={toggleDir}
              selectedFile={selectedFile}
              onSelectFile={setSelectedFile}
            />
          ))}
        </div>
      </div>

      {/* File Viewer */}
      <div className="glass border border-border/30 min-h-[400px]">
        {!selectedFile ? (
          <div className="flex flex-col items-center justify-center h-full py-12 text-center px-4">
            <FileText className="w-10 h-10 text-muted-foreground mb-4" />
            <h3 className="text-sm font-medium text-foreground mb-1">Select a file</h3>
            <p className="text-xs text-muted-foreground">
              Click on a file to view its contents
            </p>
          </div>
        ) : (
          <div className="flex flex-col h-full">
            {/* File Header */}
            <div className="flex items-center justify-between px-4 py-3 border-b border-border/30 bg-muted/20">
              <div className="flex items-center gap-2 min-w-0">
                <FileCode className="w-4 h-4 text-primary flex-shrink-0" />
                <span className="text-sm text-foreground truncate">{selectedFile}</span>
              </div>
              <button
                onClick={() => setSelectedFile(null)}
                className="p-1 text-muted-foreground hover:text-foreground transition-colors"
              >
                <X className="w-4 h-4" />
              </button>
            </div>

            {/* File Content */}
            <div className="flex-1 overflow-auto scrollbar-thin">
              {fileLoading ? (
                <div className="flex items-center justify-center py-12">
                  <Loader2 className="w-5 h-5 text-primary animate-spin" />
                </div>
              ) : fileContent?.content.type === "File" ? (
                <SyntaxHighlighter
                  style={oneDark}
                  language={getLanguageFromFilename(selectedFile)}
                  showLineNumbers
                  customStyle={{
                    margin: 0,
                    padding: "1rem",
                    background: "transparent",
                    fontSize: "0.8rem",
                    minHeight: "100%",
                  }}
                  lineNumberStyle={{
                    minWidth: "3em",
                    paddingRight: "1em",
                    color: "oklch(0.5 0.02 200)",
                    userSelect: "none",
                  }}
                >
                  {fileContent.content.content}
                </SyntaxHighlighter>
              ) : (
                <div className="p-4 text-sm text-muted-foreground">
                  Unable to display file content
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

// File tree item component
function FileTreeItem({
  item,
  depth,
  expandedDirs,
  toggleDir,
  selectedFile,
  onSelectFile,
}: {
  item: ContentItem;
  depth: number;
  expandedDirs: Set<string>;
  toggleDir: (path: string) => void;
  selectedFile: string | null;
  onSelectFile: (path: string) => void;
}) {
  const isDir = item.type === "dir";
  const isExpanded = expandedDirs.has(item.name);
  const isSelected = selectedFile === item.name;

  const handleClick = () => {
    if (isDir) {
      toggleDir(item.name);
    } else {
      onSelectFile(item.name);
    }
  };

  return (
    <div>
      <button
        onClick={handleClick}
        className={`
          w-full flex items-center gap-2 px-2 py-1.5 text-left text-sm transition-colors
          ${isSelected 
            ? "bg-primary/20 text-primary" 
            : "text-muted-foreground hover:text-foreground hover:bg-muted/30"}
        `}
        style={{ paddingLeft: `${depth * 12 + 8}px` }}
      >
        {isDir ? (
          <>
            {isExpanded ? (
              <ChevronDown className="w-3 h-3 flex-shrink-0" />
            ) : (
              <ChevronRight className="w-3 h-3 flex-shrink-0" />
            )}
            {isExpanded ? (
              <FolderOpen className="w-4 h-4 flex-shrink-0 text-primary/70" />
            ) : (
              <Folder className="w-4 h-4 flex-shrink-0 text-primary/70" />
            )}
          </>
        ) : (
          <>
            <span className="w-3" />
            <FileIcon filename={item.name} />
          </>
        )}
        <span className="truncate">{item.name}</span>
        {item.size && !isDir && (
          <span className="ml-auto text-[10px] text-muted-foreground/60">
            {formatFileSize(item.size)}
          </span>
        )}
      </button>
    </div>
  );
}

// Get file icon based on extension
function FileIcon({ filename }: { filename: string }) {
  const ext = filename.split(".").pop()?.toLowerCase();
  const codeExts = ["js", "ts", "tsx", "jsx", "py", "rs", "go", "rb", "java", "c", "cpp", "h"];
  
  if (codeExts.includes(ext || "")) {
    return <FileCode className="w-4 h-4 flex-shrink-0 text-cyan-400/70" />;
  }
  
  return <FileText className="w-4 h-4 flex-shrink-0 text-muted-foreground" />;
}

// Get syntax highlighting language from filename
function getLanguageFromFilename(filename: string): string {
  const ext = filename.split(".").pop()?.toLowerCase();
  const langMap: Record<string, string> = {
    js: "javascript",
    jsx: "jsx",
    ts: "typescript",
    tsx: "tsx",
    py: "python",
    rs: "rust",
    go: "go",
    rb: "ruby",
    java: "java",
    c: "c",
    cpp: "cpp",
    h: "c",
    hpp: "cpp",
    md: "markdown",
    json: "json",
    yaml: "yaml",
    yml: "yaml",
    toml: "toml",
    sh: "bash",
    bash: "bash",
    zsh: "bash",
    css: "css",
    scss: "scss",
    html: "html",
    xml: "xml",
    sql: "sql",
  };
  return langMap[ext || ""] || "text";
}

// Format file size
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}K`;
  return `${(bytes / (1024 * 1024)).toFixed(1)}M`;
}
