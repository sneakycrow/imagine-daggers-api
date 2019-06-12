import { Fragment } from 'react';
import Head from 'next/head';
import Layout from '../components/layout';
/** @jsx jsx */
import { css, jsx, Global } from '@emotion/core';
import styled from '@emotion/styled';

const StyledHeader = styled.div`
  min-height: 500px;
`

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
      <StyledHeader>
      <h1
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
      </h1>
      <a href="http://eepurl.com/guytNT">Sign up for the mailing list</a>
      </StyledHeader>
    </Layout>
  </Fragment>
);

export default Home;
