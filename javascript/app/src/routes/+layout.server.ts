import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({
    locals: { safeGetSession },
    cookies,
    getClientAddress,
}) => {
    const { session } = await safeGetSession();

    return {
        session,
        cookies: cookies.getAll(),
        ip: getClientAddress(),
    };
};
