import { type BaseLayoutProps } from 'fumadocs-ui/layout';

export const baseOptions: BaseLayoutProps = {
  nav: {
    title: 'Lucent',
  },
  links: [
    {
      text: 'Documentation',
      url: '/docs',
      active: 'nested-url',
    },
  ],
  githubUrl: 'https://github.com/anthropics/lucent',
};
