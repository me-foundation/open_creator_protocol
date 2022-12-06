const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');
const math = require('remark-math');
const katex = require('rehype-katex');

// With JSDoc @type annotations, IDEs can provide config autocompletion
/** @type {import('@docusaurus/types').DocusaurusConfig} */
(module.exports = {
    staticDirectories: ['static'],
    title: 'Magic Eden | Developer',
    tagline: 'Documentation for Magic Eden Open-Source Tools',
    url: 'https://magiceden-oss.github.io/',
    baseUrl: '/open_creator_protocol',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    favicon: 'img/favicon.png',
    organizationName: 'magiceden-oss',
    projectName: 'open_creator_protocol',
    scripts: [],
    stylesheets: [
        {
            href: 'https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css',
            type: 'text/css',
            integrity:
                'sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM',
            crossorigin: 'anonymous',
        },
    ],
    presets: [
        [
            '@docusaurus/preset-classic',
            /** @type {import('@docusaurus/preset-classic').Options} */
            ({
                docs: {
                    routeBasePath: '/',
                    sidebarPath: require.resolve('./sidebars.js'),
                    editUrl: 'https://github.com/magiceden-oss/open_creator_protocol/tree/main/',
                    remarkPlugins: [require('mdx-mermaid'), math],
                    showLastUpdateTime: true,
                    rehypePlugins: [katex],
                },
                theme: {
                    customCss: require.resolve('./src/css/custom.css'),
                },
            }),
        ],
    ],

    themes: [
        // ... Your other themes.
    ],

    plugins: [require.resolve('docusaurus-lunr-search')],

    themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
        ({
            tableOfContents: {
                minHeadingLevel: 2,
                maxHeadingLevel: 5,
            },
            metadata: [
                {
                    name: 'keywords',
                    content: 'solana, magic eden, ocp, open creator protocol'
                }
            ],
            colorMode: {
                defaultMode: 'dark',
                disableSwitch: false,
                respectPrefersColorScheme: false,
            },
            docs: {
                sidebar: {
                    hideable: true,
                }
            },
            navbar: {
                // title: 'Magic Eden Docs',
                logo: {
                    alt: 'Magic Eden logo',
                    src: 'img/me_logo.png',
                    srcDark: 'img/me_logo.png',
                },
                items: [
                    {
                        href: 'https://github.com/magiceden-oss/',
                        position: 'right',
                        className: 'header-github-link',
                        'aria-label': 'GitHub repository',
                    },
                ],
            },
            footer: {
                style: 'dark',
                links: [
                    {
                        title: 'Resources',
                        items: [
                            {
                                label: 'Discord',
                                href: 'https://discord.gg/magiceden',
                            },
                            {
                                label: 'Twitter',
                                href: 'https://twitter.com/magiceden',
                            },
                            {
                                label: 'StackExchange',
                                href: 'https://solana.stackexchange.com/questions/tagged/magiceden',
                            },
                            {
                                label: 'GitHub',
                                href: 'https://github.com/magiceden-oss',
                            },
                        ],
                    },
                    {
                        title: 'Powered by',
                        items: [
                            {
                                label: 'Solana',
                                href: 'https://solana.com/',
                            },
                            {
                                label: 'Docusaurus',
                                href: 'https://docusaurus.io/',
                            }
                        ],
                    },
                ],
                copyright: `Copyright © ${new Date().getFullYear()} Magic Eden`,
            },
            prism: {
                theme: lightCodeTheme,
                darkTheme: darkCodeTheme,
                additionalLanguages: ['rust'],
            },
        }),
});
