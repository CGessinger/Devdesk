<script lang="ts">
    function inWords(h, m) {
        let nums = [
            "twelve",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
            "twenty",
            "twenty one",
            "twenty two",
            "twenty three",
            "twenty four",
            "twenty five",
            "twenty six",
            "twenty seven",
            "twenty eight",
            "twenty nine",
        ];

        if (m == 0)
            return (
                "<span class='minutes'>" + nums[h] + "</span>" + " o' clock "
            );
        else if (m == 1)
            return "<span class='minutes'>one minute</span> past " + nums[h];
        else if (m == 59)
            return (
                "<span class='minutes'>one minute</span> to " +
                nums[(h % 12) + 1]
            );
        else if (m == 15)
            return "<span class='minutes'>quarter</span> past " + nums[h];
        else if (m == 30)
            return "<span class='minutes'>half</span> past " + nums[h];
        else if (m == 45)
            return (
                "<span class='minutes'>quarter</span> to " + nums[(h % 12) + 1]
            );
        else if (m <= 30)
            return (
                "<span class='minutes'>" +
                nums[m] +
                "</span>" +
                " past " +
                nums[h]
            );
        else if (m > 30)
            return (
                "<span class='minutes'>" +
                nums[60 - m] +
                "</span>" +
                " to " +
                nums[(h % 12) + 1]
            );
    }

    let words = "";
    function updateTimeout() {
        const today = new Date();
        let h = today.getHours() % 12;
        let m = today.getMinutes();
        words = inWords(h, m);

        setTimeout(updateTimeout, 1000 * 60);
    }
    function startTimeout() {
        const today = new Date();
        let h = today.getHours() % 12;
        const m = today.getMinutes();
        words = inWords(h, m);

        const seconds = today.getSeconds();
        const startIn = 60 - seconds;
        setTimeout(updateTimeout, startIn * 1000);
    }
    startTimeout();
</script>

<div class="clock">
    <span id="clock-text">{@html words}</span>
</div>

<style>
    .clock {
        margin-left: auto;
    }

    #clock-text {
        opacity: 0.8;
        text-transform: capitalize;
    }
    :global(#clock-text .minutes) {
        color: gold;
    }
</style>
