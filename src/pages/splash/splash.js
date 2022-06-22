import Head from "next/head";
import Script from "next/script";
import { useEffect } from "react";
import styles from "../../styles/Splash.module.css";

export default function Splash() {
  const load = () => {
    let loader = document.getElementById("loading-footer");
    let width = 0;
    while (width < 600)
    {
      ++width;
      loader.style.width(width)
    }
  }

  useEffect(() => {
    load();
  });

  return (
    <div style={{textTransform: "uppercase"}} className={styles["splash-container"]}>
      <Head>
        <title>GR Manager</title>
      </Head>

      {/* <Script src="/js/splash_scripts.js" /> */}

      <div className={styles["splash-container-left"]}>
        <div className={styles.centre}>
          <h1>greys revision</h1>
          <h3>powered by tauri/rust & next.js</h3>
        </div>
      </div>

      <div className={styles.right}></div>
      <div className={styles["splash-footer"]}>
        <p className={styles["splash-footer-text"]}>
          <span>
            copyright &copy; jayden andrews 2022
          </span>
        </p>
      </div>
      <div id="loading-footer" className={styles["splash-footer-loader"]}></div>
    </div>
  );
}
