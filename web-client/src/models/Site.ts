export interface Site {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    title: string;
    bucketName: string | null;
    bucketAccessKeyId: string | null;
}

export type ApiSite = Site;
