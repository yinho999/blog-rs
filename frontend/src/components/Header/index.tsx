import {
  AppBar,
  Box,
  Button,
  Container,
  Drawer,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  Toolbar,
  Typography,
} from "@mui/material";
import AdbIcon from "@mui/icons-material/Adb";
import LinkedInIcon from "@mui/icons-material/LinkedIn";
import GitHubIcon from "@mui/icons-material/GitHub";
import MenuIcon from "@mui/icons-material/Menu";
import React from "react";

const Header = () => {
  const pages = ["Blog", "About"];
  const [anchorElNav, setAnchorElNav] = React.useState<null | HTMLElement>(
    null
  );

  const handleOpenNavMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorElNav(event.currentTarget);
  };

  const handleCloseNavMenu = () => {
    setAnchorElNav(null);
  };

  const [sidebarOpen, setSidebarOpen] = React.useState(false);

  const handleToggleSidebar = () => {
    setSidebarOpen(!sidebarOpen);
  };

  return (
    <AppBar
      position="static"
      color="default"
      sx={{
        bgcolor: "grey.900",
        // height smaller when mobile
        height: { xs: 50, md: 60 },
      }}
    >
      <Container maxWidth="xl">
        <Toolbar disableGutters>
          <IconButton
            size="large"
            aria-label="open drawer"
            onClick={handleToggleSidebar}
            className="text-wooden"
            sx={{
              mr: 2,
              display: { xs: "flex", md: "none" },
            }}
          >
            <MenuIcon className="text-wooden" />
          </IconButton>
          <Typography
            variant="h6"
            noWrap
            component="a"
            href="/"
            sx={{
              mr: 2,
              display: { xs: "none", md: "flex" },
              fontFamily: "monospace",
              fontWeight: 700,
              letterSpacing: ".3rem",
              textDecoration: "none",
            }}
            className="text-wooden"
          >
            Ian
          </Typography>

          <Typography
            variant="h5"
            noWrap
            component="a"
            href="/"
            sx={{
              mr: 2,
              display: { xs: "flex", md: "none" },
              flexGrow: 1,
              fontFamily: "monospace",
              fontWeight: 700,
              letterSpacing: ".3rem",
              textDecoration: "none",
            }}
            className="text-wooden"
          >
            Ian
          </Typography>
          <Box
            sx={{ flexGrow: 1, display: { xs: "none", md: "flex" } }}
            className="text-wooden"
          >
            {pages.map((page) => (
              <Button
                key={page}
                onClick={handleCloseNavMenu}
                sx={{ my: 2, color: "white", display: "block" }}
                href={`/${page.toLowerCase()}`}
              >
                {page}
              </Button>
            ))}
          </Box>
          <Box sx={{ flexGrow: 0 }} className="text-wooden">
            <IconButton
              href="https://www.linkedin.com/in/your-linkedin-id"
              color="inherit"
            >
              <LinkedInIcon />
            </IconButton>
            <IconButton
              href="https://github.com/your-github-username"
              color="inherit"
            >
              <GitHubIcon />
            </IconButton>
          </Box>
        </Toolbar>
      </Container>
      <Drawer anchor="left" open={sidebarOpen} onClose={handleToggleSidebar}>
        <Box
          sx={{
            width: 150,
            height: "100vh",
            bgcolor: "grey.900",
          }}
          role="presentation"
          onClick={handleToggleSidebar}
          onKeyDown={handleToggleSidebar}
        >
          <List>
            {pages.map((page) => (
              <ListItem
                key={page}
                onClick={handleCloseNavMenu}
                className="text-wooden"
              >
                <ListItemButton
                  key={page}
                  onClick={handleCloseNavMenu}
                  sx={{ my: 1, display: "block" }}
                  href={`/${page.toLowerCase()}`}
                >
                  <ListItemText
                    primary={page}
                    primaryTypographyProps={{
                      textAlign: "center",
                      fontFamily: "monospace",
                      fontWeight: 700,
                    }}
                  />
                </ListItemButton>
              </ListItem>
            ))}
          </List>
        </Box>
      </Drawer>
    </AppBar>
  );
};

export default Header;
