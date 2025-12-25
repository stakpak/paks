import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: "Paks",
      url: "/",
    },
    links: [
      {
        text: "Registry",
        url: "https://paks.stakpak.dev",
        external: true,
      },
      {
        text: "GitHub",
        url: "https://github.com/stakpak/paks",
        external: true,
      },
    ],
    githubUrl: "https://github.com/stakpak/paks",
  };
}
