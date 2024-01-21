// Import the styles from the CSS Module
import styles from "./Home.module.css";
import AnimatedLetters from "../../components/AnimatedLetters";

const Home = () => {
  // Use the imported styles
  return <div className={styles.home}>This is a home.

  <AnimatedLetters letterClass={'alan'} strArray={['a','l','a','n']} index={0} />
  </div>;
};

export default Home;
