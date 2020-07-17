import { h } from 'preact';

import PageContainer from '~components/layout/PageContainer';
import Sheet from '~components/layout/Sheet';
import Stack, { StackOrientation } from '~components/layout/Stack';
import Swatch from '~components/previews/Swatch';

const COLOR_SWATCH_MATRIX = [
    ['red', 'orange', 'yellow', 'green', 'teal', 'blue', 'indigo', 'purple', 'pink'],
    ['grey', 'grey2', 'grey3', 'grey4', 'grey5', 'grey6'],
];

export default function ComponentLibraryPreview(): h.JSX.Element {
    return (
        <PageContainer>
            <Stack orientation={StackOrientation.VERTICAL}>
                <Sheet>
                    <Stack orientation={StackOrientation.VERTICAL}>
                        {COLOR_SWATCH_MATRIX.map((colorSwatchRow, key) => (
                            <Stack key={key} orientation={StackOrientation.HORIZONTAL}>
                                {colorSwatchRow.map((colorName, key) => (
                                    <Swatch key={key} colorName={colorName} />
                                ))}
                            </Stack>
                        ))}
                    </Stack>
                </Sheet>
                <Sheet>
                    <button>This is a button</button>
                </Sheet>
            </Stack>
        </PageContainer>
    );
}
