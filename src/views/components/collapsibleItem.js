import React, { useState } from 'react';
import PropTypes from 'prop-types';
import SmoothCollapse from 'react-smooth-collapse';
import styled from '@emotion/styled';

const StyledCollapsibleItem = styled.div`
  width: 100%;
`;

const StyledLabel = styled.span`
  text-decoration: underline;
  width: 100%;
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  &:hover {
    cursor: pointer;
  }
`;

const StyledSmoothCollapse = styled(SmoothCollapse)`
  width: 100%;
`;

const CollapsibleItem = ({ label, children, isExpanded }) => {
  const [isOpen, setIsOpen] = useState(isExpanded);

  return (
    <StyledCollapsibleItem>
      <StyledLabel onClick={() => setIsOpen(!isOpen)}>
        {label}
      </StyledLabel>
      <StyledSmoothCollapse expanded={isOpen}>{children}</StyledSmoothCollapse>
    </StyledCollapsibleItem>
  );
};

CollapsibleItem.propTypes = {
  label: PropTypes.string.isRequired,
  children: PropTypes.node.isRequired,
  isExpanded: PropTypes.bool
};

CollapsibleItem.defaultProps = {
  isExpanded: false
};

export default CollapsibleItem;
