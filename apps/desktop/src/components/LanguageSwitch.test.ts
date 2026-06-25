import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';

import LanguageSwitch from './LanguageSwitch.vue';
import { currentLocale, setLocale } from '@/i18n';

describe('LanguageSwitch', () => {
  it('renders a lucide language icon button and toggles locale', async () => {
    setLocale('zh-CN');

    const wrapper = mount(LanguageSwitch);
    const buttons = wrapper.findAll('button');

    expect(buttons).toHaveLength(1);
    expect(wrapper.find('svg').exists()).toBe(true);
    expect(wrapper.text()).toContain('EN');
    expect(buttons[0].attributes('aria-pressed')).toBe('false');

    await buttons[0].trigger('click');

    expect(currentLocale.value).toBe('en-US');
    expect(localStorage.getItem('axum-vue-admin.locale')).toBe('en-US');
    expect(buttons[0].attributes('aria-pressed')).toBe('true');
    expect(wrapper.text()).toContain('中文');
  });
});
