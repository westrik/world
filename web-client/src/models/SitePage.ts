export interface SitePage {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    path: string;
    siteId: string;
    noteVersionId: string;
}

export type ApiSitePage = SitePage;
