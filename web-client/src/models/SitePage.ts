export interface SitePage {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    path: string;
    published: boolean;
    siteId: string;
    noteId: string;
    noteVersionId: string;
}

export type ApiSitePage = SitePage;
