import React, { useState } from "react";
import PropTypes from "prop-types";
import {
  Card,
  CardContent,
  CardMedia,
  Typography,
  CardActionArea,
  Collapse,
  CardActions,
  Button,
  List,
  ListItem,
  ListItemText,
  Divider,
  Chip,
  Tooltip,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import MoreHorizIcon from "@mui/icons-material/MoreHoriz";
import CheckIcon from "@mui/icons-material/Check";
import CloseIcon from "@mui/icons-material/Close";
import { ExpandIconButton } from "./campaignCard.styled";

const CampaignCard = ({
  title,
  appliedFor,
  positions,
  startDate,
  endDate,
  img,
  applyClick,
}) => {
  const [expanded, setExpanded] = useState(false);

  const handleExpandClick = () => {
    setExpanded(!expanded);
  };

  return (
    <Card>
      <CardActionArea>
        <CardMedia
          component="img"
          height="140"
          image={img}
          alt={`campaign cover for ${title}`}
        />
        <CardContent>
          <Typography variant="overline" display="block">
            {`${startDate} - ${endDate}`}
          </Typography>
          <Typography variant="h6" gutterBottom>
            {title}
          </Typography>
          {appliedFor.length > 0 && (
            <>
              <Divider />
              <List>
                {appliedFor.map((position) => (
                  <ListItem disablePadding>
                    <ListItemText
                      primary={
                        // TODO CHAOS-21: use backend to determine status of user application
                        <Tooltip
                          title={
                            position === "Careers Director"
                              ? "Processing application"
                              : position === "Socials Director"
                              ? "Position offered"
                              : "Application rejected"
                          }
                        >
                          <Chip
                            color={
                              position === "Careers Director"
                                ? "warning"
                                : position === "Socials Director"
                                ? "success"
                                : "error"
                            }
                            icon={
                              position === "Careers Director" ? (
                                <MoreHorizIcon />
                              ) : position === "Socials Director" ? (
                                <CheckIcon />
                              ) : (
                                <CloseIcon />
                              )
                            }
                            label={position}
                          />
                        </Tooltip>
                      }
                    />
                  </ListItem>
                ))}
              </List>
            </>
          )}
        </CardContent>
      </CardActionArea>

      <CardActions disableSpacing>
        <Button
          size="small"
          variant="outlined"
          onClick={
            applyClick /* CHAOS-51: integrate w/ backend to navigate to specific page */
          }
        >
          Apply
        </Button>
        <ExpandIconButton
          expanded={expanded}
          onClick={handleExpandClick}
          aria-expanded={expanded}
          aria-label="show more"
        >
          <ExpandMoreIcon />
        </ExpandIconButton>
      </CardActions>

      <Collapse in={expanded} timeout="auto" unmountOnExit>
        <CardContent>
          <List>
            {positions.map((position) => (
              <ListItem disablePadding>
                <ListItemText
                  primary={
                    <Chip
                      variant="outlined"
                      label={`${position.number}x ${position.name}`}
                    />
                  }
                />
              </ListItem>
            ))}
          </List>
        </CardContent>
      </Collapse>
    </Card>
  );
};

CampaignCard.propTypes = {
  title: PropTypes.string.isRequired,
  appliedFor: PropTypes.arrayOf(PropTypes.string).isRequired,
  positions: PropTypes.arrayOf(
    PropTypes.shape({
      name: PropTypes.string,
      number: PropTypes.number,
    })
  ).isRequired,
  startDate: PropTypes.string.isRequired,
  endDate: PropTypes.string.isRequired,
  img: PropTypes.string.isRequired,
  applyClick: PropTypes.func.isRequired,
};

export default CampaignCard;
