import { AuthContext } from '~auth/AuthContext';
import { ApiResponse, request, RequestMethod } from '~utils/network';
import { ApiSitePage, SitePage } from '~models/SitePage';

export interface GetSitePagesResponse extends ApiResponse {
    sitePages: Array<ApiSitePage>;
}

export default async function listSitePages(
    authContext: AuthContext,
    siteId: string,
    onReceiveResponse: (notes: Array<SitePage>) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetSitePagesResponse>(RequestMethod.GET, `/site/${siteId}/page`, authContext);
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.sitePages.map((apiPage): SitePage => apiPage));
    }
}
