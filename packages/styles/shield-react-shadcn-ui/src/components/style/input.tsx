import type { Input as ApiInput } from '@rustforweb/shield-react';
import { useId, useMemo } from 'react';

// import { Controller } from 'react-hook-form';

import { Button } from '../ui/button.js';
import { Field, FieldLabel } from '../ui/field.js';
import { Input } from '../ui/input.js';

export type StyleInputProps = {
    // control: Control;
    input: ApiInput;
};

export const StyleInput = ({ input }: StyleInputProps) => {
    const reactId = useId();
    const id = useMemo(() => `${reactId}-${input.name}`, [reactId]);

    if (input.type.type === 'button' || input.type.type === 'reset' || input.type.type === 'submit') {
        return (
            <Button name={input.name} type={input.type.type} variant="outline">
                {input.value}
            </Button>
        );
    }

    // TODO: Handle different input types.

    return (
        // <Controller
        //     name={input.name}
        //     control={control}
        //     render={({ field, fieldState }) => (
        //         <Field data-invalid={fieldState.invalid}>
        //             {input.label && <FieldLabel htmlFor={id}>{input.label}</FieldLabel>}
        //             <Input
        //                 {...field}
        //                 id={id}
        //                 type={input.type.type}
        //                 placeholder={input.label ?? undefined}
        //                 aria-invalid={fieldState.invalid}
        //             />
        //             {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
        //         </Field>
        //     )}
        // />

        <Field>
            {input.label && <FieldLabel htmlFor={id}>{input.label}</FieldLabel>}
            <Input
                id={id}
                name={input.name}
                type={input.type.type}
                placeholder={input.label ?? undefined}
                value={input.value ?? undefined}
            />
        </Field>
    );
};
