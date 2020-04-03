import { h } from 'preact';

interface Props {
    title: string;
    children?: Array<h.JSX.Element>;
    fixed: boolean;
    backLink?: string;
    backLinkTitle?: string;
}

export default function Header(props: Props): h.JSX.Element | null {
    return null;
    // return (
    //     // <div className={`header ${props.fixed ? 'fixed' : ''}`}>
    //     <div className={`header`}>
    //         <div>
    //             {props.backLink ? (
    //                 <a className="back-link" href={props.backLink}>
    //                     &larr; {props.backLinkTitle ? props.backLinkTitle : 'back'}
    //                 </a>
    //             ) : null}
    //             <h1>{props.title}</h1>
    //         </div>
    //         {props.children ? (
    //             <div className="header-element-list">
    //                 {props.children.map((child, key) => (
    //                     <div key={key} className="header-element">
    //                         {child}
    //                     </div>
    //                 ))}
    //             </div>
    //         ) : null}
    //     </div>
    // );
}
