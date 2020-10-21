import { h } from 'preact';
import Button, { ButtonSize, ButtonVariant } from '~components/Button';

export default { title: 'Button' };

export function normal(): h.JSX.Element {
    return (
        <ul className="story-container">
            {[ButtonVariant.PRIMARY, ButtonVariant.SECONDARY, ButtonVariant.TERTIARY].map((variant, key) => {
                {
                    return [ButtonSize.LARGE, ButtonSize.MEDIUM, ButtonSize.SMALL, ButtonSize.XSMALL].map(
                        (size, key2) => {
                            return (
                                <li key={key * key2}>
                                    <Button variant={variant} size={size} title={`Test Button (${variant}, ${size})`} />
                                </li>
                            );
                        },
                    );
                }
            })}
        </ul>
    );
}

// export function withPopover(): h.JSX.Element {
//     const showPopper = (): void => {
//         console.log('TODO: show popper');
//     };
//     return (
//         <div className="story-container">
//             <div>
//                 <Popover key="popover" />
//             </div>
//             <div>
//                 <Button onClick={showPopper} size={ButtonSize.LARGE} title="large" />
//             </div>
//             <div>
//                 <Button onClick={showPopper} title="default" />
//             </div>
//             <div>
//                 <Button onClick={showPopper} size={ButtonSize.SMALL} title="small" />
//             </div>
//             <div>
//                 <Button onClick={showPopper} size={ButtonSize.XSMALL} title="x-small" />
//             </div>
//         </div>
//     );
// }
