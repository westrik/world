import { useEffect, useMemo } from 'preact/hooks';

const observerConfig = {
    attributes: true,
    characterData: true,
    childList: true,
    attributeOldValue: true,
    subtree: true,
};

export default function useMutationObserver(
    isActive: boolean,
    elementId: string,
    callback: (mutations: Array<MutationRecord>) => void,
): void {
    const observer = useMemo(() => new MutationObserver(callback), [callback]);

    useEffect(() => {
        if (isActive) {
            const targetNode = document.getElementById(elementId);
            if (targetNode) {
                observer.observe(targetNode, observerConfig);
            }
        } else {
            observer.disconnect();
        }

        return () => observer.disconnect();
    }, [isActive, observer]);
}
