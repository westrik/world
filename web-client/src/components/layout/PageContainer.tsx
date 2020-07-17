import { h } from 'preact';

interface PageContainerProps {
    children: h.JSX.Element;
}

export default function PageContainer(props: PageContainerProps): h.JSX.Element {
    return (
        <main className="page-container" role="main">
            {props.children}
        </main>
    );
}
