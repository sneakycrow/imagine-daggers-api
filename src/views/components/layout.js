import styled from '@emotion/styled';

const StyledLayout = styled.main`
  display: grid;
  min-height: 100vh;
  grid-template-rows: 20% 1fr 10%;
  grid-template-columns: repeat(12, 1fr);
`;

const Layout = ({ children }) => (
  <StyledLayout>
    {children}
  </StyledLayout>
)

export default Layout;
