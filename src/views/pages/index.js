import { Fragment } from 'react';
import Head from 'next/head';
import Layout from '../components/layout';
import CollapsibleItem from '../components/collapsibleItem';
/** @jsx jsx */
import { css, jsx, Global } from '@emotion/core';
import styled from '@emotion/styled';

const StyledHeader = styled.h1`
  min-height: 100%;
`;

const StyledBody = styled.div`
  grid-row: 2;
  grid-column: 2 / span 10;
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const Home = () => (
  <Fragment>
    <Head>
      <link
        rel="stylesheet"
        href="https://cdnjs.cloudflare.com/ajax/libs/modern-normalize/0.5.0/modern-normalize.min.css"
      />
      <link
        href="https://fonts.googleapis.com/css?family=Noto+Sans+HK|Slabo+27px&display=swap"
        rel="stylesheet"
      />

      <title>Imagine Daggers</title>
    </Head>
    <Global
      css={css`
        h1 {
          font-family: 'Noto Sans HK', sans-serif;
        }
      `}
    />
    <Layout>
      <StyledHeader
        css={css`
          grid-row: 1;
          grid-column: 2 / span 10;
          text-align: center;
          text-transform: uppercase;
          font-family: 'Noto Sans HK', sans-serif;
          font-size: 64px;
        `}
      >
        Imagine Daggers
      </StyledHeader>
      <StyledBody>
        <a href="http://eepurl.com/guytNT" css={css`margin-bottom: 24px;`}>Sign up for the mailing list</a>
        <CollapsibleItem label="What is Imagine Daggers?" isExpanded>
          <p>
            Imagine Daggers is a project for DnD players. It is a web application for turning your
            session recordings into short stories. It allows you to connect with various artists as
            well so that you can hire them for commissions for your DnD group. After your campaign
            finishes, you can compile all your stories together to create a book of your campaign.
          </p>
          <p>Ultimately, we want to help you breath more life into your adventures</p>
        </CollapsibleItem>
      </StyledBody>
    </Layout>
  </Fragment>
);

export default Home;
