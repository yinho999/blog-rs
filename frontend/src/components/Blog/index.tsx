// import React from "react";

import { Paper, Typography, Link } from "@mui/material";

const Blog = ({
  title,
  description,
  link,
}: {
  title: string;
  description: string;
  link: string;
}) => {
  return (
    <Paper elevation={3} sx={{ padding: 3, marginBottom: 2 }}>
      <Typography variant="h5" component="h3" gutterBottom>
        {title}
      </Typography>
      <Typography variant="body1" paragraph>
        {description}
      </Typography>
      <Link href={link} underline="hover">
        Read More
      </Link>
    </Paper>
  );
};
export default Blog;
