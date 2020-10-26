export interface Site {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    title: string;
    bucketDomainName: string | null;
    bucketAccessKeyId: string | null;
}

export type ApiSite = Site;
