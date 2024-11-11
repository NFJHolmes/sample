import { zodResponseFormat } from 'openai/helpers/zod';
import { z } from 'zod';

const Implementation = z.object({
    number: z.number(),
    description: z.string(),
});

const Insight = z.object({
    title: z.string(),
    explanation: z.string(),
    implementation: z.array(Implementation),
});

const Insights = z.object({
    insights: z.object({
        first: Insight,
        second: Insight,
        third: Insight,
    }),
    summary: z.string(),
});

export const InsightsResponseFormat = zodResponseFormat(Insights, 'insights');
export type InsightsResponseFormat = z.infer<typeof Insights>;
export type InsightType = z.infer<typeof Insight>;
export type InsightsType = z.infer<typeof Insights>;
