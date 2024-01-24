import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
// import Home from "../pages/Home";
// import Blog from "../pages/Blog";
// import BlogList from "../pages/BlogList";
import Layout from "../components/Layout";
import About from "../components/About";
import Contact from "../components/Contact";
import NewHome from "../components/Home"

const AppRouter = () => {
  return (
    <Router>
      {/*<Routes>*/}
      {/*  <Route path="/" element={<Home />} />*/}
      {/*  <Route path="/blog" element={<BlogList />} />*/}
      {/*  <Route path="/blog/:slug" element={<Blog />} />*/}
      {/*</Routes>*/}
        <Routes>
            <Route path="/" element={<Layout />}>
                <Route index element={<NewHome />} />
                <Route path="about" element={<About />} />
                <Route path="contact" element={<Contact />} />
            </Route>
        </Routes>
    </Router>
  );
};

export default AppRouter;
