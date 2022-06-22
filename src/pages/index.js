/* eslint-disable @next/next/no-img-element */
import Head from "next/head";
import MarkdownEditor from '../components/markdown-editor/markdown-editor';
import styles from "../styles/Home.module.css";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Home() {
  const safe_close_splash = () => {
    const sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));

    sleep(2000).then(() => {
      invoke("safe_close_splashscreen");
    });
  };

  useEffect(() => {
    // safe_close_splash("safe_close_splashscreen");
    invoke("safe_close_splashscreen");
  });

  return (
    <div className={styles.container}>
      <Head>
        <title>Greys Revision Manager</title>
      </Head>

      <MarkdownEditor />
 
      <footer className={styles.footer}>
          Powered by{" "}
          <span className={styles.logo}>
            <img
              src={require("/public/vercel.svg?original")}
              alt="Vercel Logo"
              width={72}
              height={16}
            />
          </span>
      </footer>
    </div>
  );
}
