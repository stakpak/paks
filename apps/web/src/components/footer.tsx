import { Github, BookOpen, ExternalLink } from "lucide-react";

export function Footer() {
  return (
    <footer className="relative border-t border-border/30 bg-card/30">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-12">
        <div className="flex flex-col md:flex-row items-center justify-between gap-6">
          {/* Branding */}
          <div className="flex flex-col items-center md:items-start gap-2">
            <span className="text-xl font-bold gradient-text">Paks</span>
            <p className="text-sm text-muted-foreground">
              Made with <span className="text-primary">♥</span> by{" "}
              <a 
                href="https://stakpak.dev" 
                target="_blank" 
                rel="noopener noreferrer"
                className="text-foreground hover:text-primary transition-colors inline-flex items-center gap-1"
              >
                Stakpak
                <ExternalLink className="w-3 h-3" />
              </a>
            </p>
          </div>

          {/* Links */}
          <div className="flex items-center gap-6">
            <a 
              href="https://github.com/stakpak/paks" 
              target="_blank" 
              rel="noopener noreferrer"
              className="flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors group"
            >
              <Github className="w-4 h-4 group-hover:text-primary transition-colors" />
              GitHub
            </a>
            <a 
              href="https://github.com/stakpak/paks?tab=readme-ov-file#-paks" 
              target="_blank" 
              rel="noopener noreferrer"
              className="flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors group"
            >
              <BookOpen className="w-4 h-4 group-hover:text-primary transition-colors" />
              Docs
            </a>
          </div>
        </div>

        {/* Copyright */}
        <div className="mt-8 pt-6 border-t border-border/20 text-center">
          <p className="text-xs text-muted-foreground/70">
            © {new Date().getFullYear()} Stakpak. Apache 2.0 License.
          </p>
        </div>
      </div>
    </footer>
  );
}
