import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

// Initialize custom fonts
const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

// Metadata including custom favicon
export const metadata: Metadata = {
  title: "Qompass AI", // Set the title of your site
  description: "Your quantum compass for AI exploration.",
  icons: {
    icon: "/qompass.png", // Link to your custom favicon (ensure it's in the public directory)
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        {/* Link to the custom favicon */}
        <link rel="icon" href="/qompass.png" /> {/* Make sure this path is correct */}
      </head>
      <body className={`${geistSans.variable} ${geistMono.variable} antialiased`}>
        {children}
      </body>
    </html>
  );
}

