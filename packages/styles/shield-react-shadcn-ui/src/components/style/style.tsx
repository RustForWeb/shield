import type { ReactStyleProps } from '@rustforweb/shield-react';

import { StyleForm } from './form.js';

export const ShadcnUiStyle = ({ action }: ReactStyleProps) => {
    return (
        <div className="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
            <div className="flex w-full max-w-sm flex-col gap-10">
                {/* TODO: Logo */}

                <div className="flex flex-col gap-2">
                    <h1 className="text-2xl font-bold">{action.name}</h1>

                    {action.methodForms.flatMap((methodForm) =>
                        methodForm.providerForms.flatMap((providerForm) => (
                            <StyleForm
                                key={`${action.id}-${methodForm.id}-${providerForm.id}`}
                                actionId={action.id}
                                methodId={methodForm.id}
                                providerId={providerForm.id}
                                form={providerForm.form}
                            />
                        )),
                    )}
                </div>
            </div>
        </div>
    );
};
