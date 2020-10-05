import { AuthContext } from '~auth/AuthContext';
import { ApiSite, Site } from '~models/Site';
import { ApiResponse, request, RequestMethod } from '~utils/network';

export interface GetSitesResponse extends ApiResponse {
    sites: Array<ApiSite>;
}

export default async function listSites(
    authContext: AuthContext,
    onReceiveResponse: (notes: Array<Site>) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetSitesResponse>(RequestMethod.GET, '/site', authContext);
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.sites.map((apiSite): Site => apiSite));
    }
}
