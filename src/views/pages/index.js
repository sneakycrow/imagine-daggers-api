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
      </StyledHeader>
      <footer
        css={css`
          grid-row: 3;
          grid-column: 2 / span 10;
          text-align: center;
          display: flex;
          justify-content: center;
          align-items: center;
          flex-direction: column;
          > * {
            font-size: 10px;
          }
        `}
      >
        <h6 css={css`
          margin-bottom: 8px;
        `}>Credits</h6>
        <div>
          Icons made by{' '}
          <a href="https://www.freepik.com/" title="Freepik">
            Freepik
          </a>{' '}
          from{' '}
          <a href="https://www.flaticon.com/" title="Flaticon">
            www.flaticon.com
          </a>{' '}
          is licensed by{' '}
          <a
            href="http://creativecommons.org/licenses/by/3.0/"
            title="Creative Commons BY 3.0"
            target="_blank"
          >
            CC 3.0 BY
          </a>
        </div>
      </footer>
    </Layout>
  </Fragment>
);

export default Home;
