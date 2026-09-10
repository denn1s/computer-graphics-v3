import type { Metadata } from "next";
import { IBM_Plex_Mono, Manrope } from "next/font/google";
import "./globals.css";

const body = Manrope({ variable: "--font-body", subsets: ["latin"] });
const mono = IBM_Plex_Mono({ variable: "--font-mono", weight: ["400", "500", "600"], subsets: ["latin"] });

export const metadata: Metadata = {
  title: "From Random Numbers to Minecraft Terrain",
  description: "An interactive field guide to seeds, coherent noise, FBM, heightfields, and Minecraft-inspired terrain generation.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body className={`${body.variable} ${mono.variable}`}>{children}</body></html>;
}
